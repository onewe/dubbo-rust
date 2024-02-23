use std::time::SystemTime;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse::ParseStream;
use syn::{Attribute, FnArg, Signature, Token};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use crate::common::{Deserialization, Serialization};

pub struct RpcCall {
    attrs: Vec<Attribute>,
    sig: Signature,
}

impl syn::parse::Parse for RpcCall {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let func = syn::ImplItemFn::parse(input)?;
        Ok(RpcCall {
            attrs: func.attrs,
            sig: func.sig,
        })
    }

}


impl RpcCall {
    
    pub fn to_token_stream(self, rpc_call_meta_info: RpcCallMetaInfo) -> TokenStream {
        let RpcCall {
            attrs,
            sig,
        } = self;
        
        TokenStream::default()
    }
    
    fn gen_proxy_method( attrs: Vec<Attribute>,sig: Signature,rpc_call_meta_info: RpcCallMetaInfo) -> TokenStream {
        let Signature {
            ident: fn_ident,
            generics,
            inputs,
            output,
            ..
        } = sig;
        
        let (target_method_name, reference_info) = RpcCall::extract_target_method_name_and_reference(fn_ident);
        
        

        quote! {
           #(#attrs)*
            pub async fn #target_method_name
        }
    }
    
    fn extract_target_method_name_and_reference(fn_ident: Ident) -> (String, String) {
        let proxy_method_name = fn_ident.to_string();
        let mut vec: Vec<String> = proxy_method_name.split("_proxy_by_").map(|s|s.to_owned()).collect();
        if vec.len() != 2 {
            panic!("proxy method name is not valid");
        }
        
        let target_method_name = vec.pop();
        let reference_info = vec.pop();
        
        (target_method_name.expect("target method name is not valid"), reference_info.expect("reference info is not valid"))
        
    }
    
    fn assert_input_args(input: &Punctuated<FnArg, Token![,]>, rpc_call_meta_info: &RpcCallMetaInfo) -> syn::Result<TokenStream>{
        
        if input.is_empty() {
            return Err(syn::Error::new(input.span(), "arguments must not be empty"));
        }
        
        match rpc_call_meta_info.rpc_type {
            RpcType::ClientStream => {
                if input.len() != 2 {
                    return Err(syn::Error::new(input.span(), "client stream rpc call must have only one argument"));
                }
            },
            RpcType::BiStream => {
                if input.len() != 2 {
                    return Err(syn::Error::new(input.span(), "bi stream rpc call must have only one argument"));
                }
            },
            _ => {}
        }
        
        
        
        let mut assert_stream_type = Vec::default();
        
        let mut contains_self = false;
        for arg in input {
            match arg {
                FnArg::Receiver(receiver) => {
                    if receiver.mutability.is_none() {
                        return Err(syn::Error::new(receiver.span(), "receiver must be mutable"));
                    }
                    contains_self = true;
                }
                FnArg::Typed(pat_type) => {
                   let arg_type = &pat_type.ty;
                   
                    match rpc_call_meta_info.rpc_type {
                       RpcType::ClientStream | RpcType::BiStream => {
                           let arg_type = &pat_type.ty;
                           assert_stream_type.push(quote! {
                              
                           });
                       },
                       _ => {}
                   }
                }
            }
        }
        
        
        Ok(TokenStream::default())
    }
        
        
}


pub struct RpcCallMetaInfo {
    rpc_type: RpcType,
    serialization: Serialization,
    deserialization: Deserialization,
}

impl RpcCallMetaInfo {
    
    pub fn rpc_type(&self) -> RpcType {
        self.rpc_type.clone()
    }
    
    pub fn serialization(&self) -> Serialization {
        self.serialization.clone()
    }
    
    pub fn deserialization(&self) -> Deserialization {
        self.deserialization.clone()
    }
}


impl syn::parse::Parse for RpcCallMetaInfo {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut rpc_type = RpcType::Unary;
        let mut serialization = Serialization::Json;
        let mut deserialization = Deserialization::Json;
        while !input.is_empty() {
            let key: syn::Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let value: syn::LitStr = input.parse()?;
            let value = value.value();
            match key.to_string().as_str() {
                "type" => {
                    rpc_type = RpcType::from(value);
                }
                "ser" => {
                    serialization = Serialization::from(value);
                }
                "deser" => {
                    deserialization = Deserialization::from(value);
                }
                _ => {
                    return Err(syn::Error::new(key.span(), "unknown attribute"));
                }
            }

            let _ = input.parse::<Token![,]>();
        }
        
        Ok(RpcCallMetaInfo {
            rpc_type,
            serialization,
            deserialization
        })
    }
}


#[derive(Clone)]
pub enum RpcType {
    Unary, 
    ClientStream, 
    ServerStream, 
    BiStream
}

impl From<String> for RpcType {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "unary" => RpcType::Unary,
            "client_stream" => RpcType::ClientStream,
            "server_stream" => RpcType::ServerStream,
            "bi_stream" => RpcType::BiStream,
            _ => RpcType::Unary
        }
    }
}