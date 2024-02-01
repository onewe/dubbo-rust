use std::{sync::Arc, task::Poll};

use dubbo_base::Url;
use futures::future::poll_fn;
use thiserror::Error;
use tokio::{sync::{mpsc, oneshot, Notify, watch}, select};
use tracing::{error, info, debug};

use crate::{StdError};

use super::{InvokeError, Invoker, RpcInvocation, RpcResponse};

const DEFAULT_CHANNEL_SIZE: usize = 64;

pub struct CloneableInvoker {
    url: Url,
    sender: mpsc::Sender<(RpcInvocation, oneshot::Sender<Result<RpcResponse, StdError>>)>,
    close: Arc<Notify>,
    gate: InvokerGate,
}

impl CloneableInvoker {

    pub fn new(mut invoker: Box<dyn Invoker + Send + 'static>) -> Self {

        let (gate, gate_guard) = InvokerGate::new();

        let close = Arc::new(Notify::new());

        let close_clone = close.clone();

        let url = invoker.url().clone();

        let (sender, mut receiver) = mpsc::channel::<(RpcInvocation, oneshot::Sender<Result<RpcResponse, StdError>>)>(DEFAULT_CHANNEL_SIZE);

        tokio::spawn(async move {
            let url = invoker.url().clone();
            loop {
                select! {
                    _ = close.notified() => {
                        info!("invoker is destroyed: {}", url);
                        return;
                    },
                    receive_message = receiver.recv() => match receive_message {
                        Some((invocation, tx)) => {
                            let ready = {
                                let mut poll_ready = invoker.ready();
                                poll_fn(|cx| {
                                    match poll_ready.as_mut().poll(cx) {
                                        Poll::Ready(Ok(_)) => {
                                            debug!("clone invoker is ready: {}", url);
                                            match gate_guard.open() {
                                                Ok(_) => {
                                                    Poll::Ready(Ok(()))
                                                },
                                                Err(e) => {
                                                    error!("open gate error: {}", e);
                                                    Poll::Ready(Err(e))
                                                }
                                            }
                                        },
                                        Poll::Ready(Err(e)) => {
                                            error!("clone invoker is not ready, occur an error: {}", e);
                                            match gate_guard.shut() {
                                                Ok(_) => {
                                                    Poll::Ready(Err(e))
                                                },
                                                Err(e) => {
                                                    error!("shut gate error: {}", e);
                                                    Poll::Ready(Err(e))
                                                }
                                            }
                                        },
                                        Poll::Pending => {
                                            debug!("clone invoker is not ready, current state is pending: {}", url);
                                            match gate_guard.shut() {
                                                Ok(_) => {
                                                    Poll::Pending
                                                },
                                                Err(e) => {
                                                    error!("shut gate error: {}", e);
                                                    Poll::Ready(Err(e))
                                                }
                                            }
                                        }
                                    }
                                }).await
                            };

                            match ready {
                                Ok(_) => {},
                                Err(e) => {
                                    error!("clone invoker is not ready, occur an error: {}", e);
                                    continue;
                                }
                            }

                            match tx.send(invoker.invoke(invocation).await) {
                                Ok(_) => {},
                                Err(e) => {
                                    match e {
                                        Err(e1) => error!("send response error: {}", e1),
                                       _ => {},
                                    }
                                }
                            }
                        },
                        None => {
                            info!("message send end was closed, so invoker was destroyed: {}", invoker.url());
                            return;
                        }
                    }
                }
            }
        });

       Self { url, sender, close: close_clone, gate}
    }
}

impl Drop for CloneableInvoker {

    fn drop(&mut self) {
       let ref_count = Arc::strong_count(&self.close);
       if ref_count == 2 {
           self.close.notify_one();
       }
    }
}

impl Clone for CloneableInvoker {
    fn clone(&self) -> Self {
        Self {
            url: self.url.clone(),
            sender: self.sender.clone(),
            close: self.close.clone(),
            gate: self.gate.clone(),
        }
    }
}

#[async_trait::async_trait]
impl Invoker for CloneableInvoker {

    async fn ready(&mut self) -> Result<(), StdError> {
       loop {
            if self.gate.is_open() {
                return Ok(())
            }

            match self.gate.wait_change().await {
                Ok(_) => {},
                Err(e) => {
                    error!("wait gate change error: {}", e);
                    return Err(e);
                }
            }
       }
    }

    async fn invoke(&mut self, invocation: RpcInvocation) -> Result<RpcResponse, StdError> {
        if self.gate.is_shut() {
            return Err(InvokeError::new(format!("gate is shut: {}", self.url)).into());
        }

        let (tx, rx) = oneshot::channel::<Result<RpcResponse, StdError>>();
        match self.sender.send((invocation, tx)).await {
            Ok(_) => {},
            Err(e) => {
                error!("send invocation error: {}", e); 
                return Err(InvokeError::new(format!("send invocation error: {}", e)).into());
            }
        }
        rx.await?
    }

    fn url(&self) -> &Url {
        &self.url
    }
}

#[derive(Clone)]
pub enum GateState {
    Open,
    Shut
}


pub struct InvokerGateGuard {
    state: watch::Sender<GateState>
}

impl InvokerGateGuard {

    pub fn new(state: watch::Sender<GateState>) -> Self {
        Self { state }
    }

    pub fn open(&self) -> Result<(), StdError>{
        match self.state.send(GateState::Open) {
            Ok(_) => Ok(()),
            Err(e) => Err(InvokerGateError::new(format!("open gate error: {}", e)).into())
        
        }
    }

    pub fn shut(&self) -> Result<(), StdError>{
        match self.state.send(GateState::Shut) {
            Ok(_) => Ok(()),
            Err(e) => Err(InvokerGateError::new(format!("shut gate error: {}", e)).into())
        
        }
    }
   
}


#[derive(Clone)]
pub struct InvokerGate {
    state: watch::Receiver<GateState>
    
}

impl InvokerGate {

    pub fn new() -> (Self, InvokerGateGuard) {
        let (tx, rx) = watch::channel::<GateState>(GateState::Open);
        let gate_guard = InvokerGateGuard::new(tx);
        
        (Self { state: rx }, gate_guard)
    }
   
    pub fn state(&self) -> GateState {
        self.state.borrow().clone()
    }

    pub fn is_open(&self) -> bool {
        matches!(self.state(), GateState::Open)
    }

    pub fn is_shut(&self) -> bool {
        matches!(self.state(), GateState::Shut)
    }

    pub async fn wait_change(&mut self) -> Result<(), StdError>{
        match self.state.changed().await {
            Ok(_) => Ok(()),
            Err(e) => Err(InvokerGateError::new(format!("wait change error: {}", e)).into())
        }
    }
}


#[derive(Error, Debug)]
#[error("invoker gate error: {0}")]
pub struct InvokerGateError(String);

impl InvokerGateError {

    pub fn new(msg: String) -> Self {
        Self(msg)
    }
}