use futures_core::future::BoxFuture;
use tower::{discover::ServiceList, ServiceExt};
use tower_service::Service;

use crate::{
    codegen::RpcInvocation,
    invoker::{clone_body::CloneBody, clone_invoker::CloneInvoker},
    param::Param,
    svc::NewService,
    StdError,
};

use crate::protocol::triple::triple_invoker::TripleInvoker;
