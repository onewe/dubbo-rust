use std::sync::Arc;

use tokio::{sync::{mpsc, oneshot, Notify}, select};
use tracing::{error, info};

use crate::{StdError, url::Url};

use super::{RpcInvocation, Invoker, RpcResponse};

const DEFAULT_CHANNEL_SIZE: usize = 64;

pub struct CloneableInvoker {
    url: Url,
    sender: mpsc::Sender<(RpcInvocation, oneshot::Sender<Result<RpcResponse, StdError>>)>,
    close: Arc<Notify>
}

impl CloneableInvoker {

    pub fn new(mut invoker: Box<dyn Invoker + Send + 'static>) -> Self {

        let close = Arc::new(Notify::new());

        let close_clone = close.clone();

        let url = invoker.url().clone();

        let (sender, mut receiver) = mpsc::channel::<(RpcInvocation, oneshot::Sender<Result<RpcResponse, StdError>>)>(DEFAULT_CHANNEL_SIZE);

        tokio::spawn(async move {
            loop {
                select! {
                    _ = close.notified() => {
                        info!("invoker was destroyed: {}", invoker.url());
                        return;
                    },
                    receive_message = receiver.recv() => match receive_message {
                        Some((invocation, tx)) => {
                            match tx.send(invoker.invoke(invocation).await) {
                                Ok(_) => {},
                                Err(e) => {
                                    match e {
                                        Err(e1) => error!("send response error: {}", e1),
                                        Ok(resp) => error!("send response error: {}", resp),
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

       Self { url, sender, close: close_clone }
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
        }
    }
}
