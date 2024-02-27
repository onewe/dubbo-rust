use std::time::SystemTime;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::parse::ParseStream;
use syn::{Attribute, FnArg, Signature, Token};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use crate::common::{Deserialization, Serialization};
use crate::reference_meta_info::ReferenceMetaInfo;

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
    
    pub fn to_token_stream(self, rpc_call_meta_info: RpcCallMetaInfo) -> syn::Result<TokenStream> {
        let RpcCall {
            attrs,
            sig,
        } = self;
        
        Ok(TokenStream::default())
    }
    
    fn gen_proxy_method( attrs: Vec<Attribute>,sig: Signature, mut rpc_call_meta_info: RpcCallMetaInfo) -> syn::Result<TokenStream> {
        let Signature {
            ident: fn_ident,
            generics,
            inputs,
            output,
            ..
        } = sig;
        let (_, ty_generics, where_clause) = generics.split_for_impl();
      
        
        let (target_method_name, reference_info) = RpcCall::extract_target_method_name_and_reference(fn_ident);
        let global_reference_meta_info = ReferenceMetaInfo::get_from_global_by_reference_name(&reference_info);
        
        if rpc_call_meta_info.serialization.is_none(){
                match global_reference_meta_info {
                    Some(ref meta_info) => {
                        rpc_call_meta_info.serialization = Some(meta_info.serialization());
                    },
                    None => {
                        rpc_call_meta_info.serialization = Some(Serialization::Json);
                    }
                }
        }
        
        if rpc_call_meta_info.deserialization.is_none() {
            match global_reference_meta_info {
                Some(ref meta_info) => {
                    rpc_call_meta_info.deserialization = Some(meta_info.deserialization());
                },
                None => {
                    rpc_call_meta_info.deserialization = Some(Deserialization::Json);
                }
            }
        }
        
        let assert_type_token_stream = RpcCall::assert_input_args(&inputs, &rpc_call_meta_info)?;

        Ok(quote! {
           #(#attrs)*
            pub async fn #target_method_name #ty_generics(#inputs) #output #where_clause {
                #assert_type_token_stream
                todo!()
            }
        })
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
        
        let Some(ser) = rpc_call_meta_info.serialization.clone() else {
            return Err(syn::Error::new(input.span(), "can not found serialization configuration"));
        };
        
        
        let mut assert_type_token_stream = TokenStream::default();
        
        let mut contain_mut_self = false;
        for arg in input {
            match arg {
                FnArg::Receiver(receiver) => {
                    if receiver.mutability.is_none() {
                        return Err(syn::Error::new(receiver.span(), "receiver must be mutable"));
                    }
                    contain_mut_self = true;
                }
                FnArg::Typed(pat_type) => {
                   let arg_type = &pat_type.ty;
                    match rpc_call_meta_info.rpc_type {
                       RpcType::ClientStream | RpcType::BiStream  => {
                           let arg_type = &pat_type.ty;
                           match &ser {
                               Serialization::Json => {
                                  
                                   assert_type_token_stream.extend(quote! {
                                      mod _assert_input_type {
                                           fn check_cs_arg_type<T>() where T: ::futures::Stream, ::dubbo::serialize::SerdeJsonSerialization<<T as ::futures::Stream>::Item>: ::dubbo::serialize::Serializable, {}
                                           fn test_fn() {check_cs_arg_type<#arg_type>();}
                                       }
                                   });
                               },
                               Serialization::Protobuf => {

                                   assert_type_token_stream.extend(quote! {
                                       mod _assert_input_type {
                                           fn check_cs_arg_type<T>() where T: ::futures::Stream, ::dubbo::serialize::ProstSerialization<<T as ::futures::Stream>::Item>: ::dubbo::serialize::Serializable, {}
                                           fn test_fn() {check_cs_arg_type<#arg_type>();}
                                       }
                                       
                                   });
                                   
                                   
                               },
                               Serialization::Other(_) => {
                                   assert_type_token_stream.extend(quote! { 
                                       mod _assert_input_type {
                                           fn check_cs_arg_type<T>() where T: ::futures::Stream, <T as ::futures::Stream>::Item: ::dubbo::serialize::Serializable, {}
                                           fn test_fn() {check_cs_arg_type<#arg_type>();}
                                       }
                                   });
                                  
                               }
                           }
                           
                        }
                        RpcType::Unary | RpcType::ServerStream => {
                           match &ser {
                               Serialization::Json => {
                                   assert_type_token_stream.extend(quote! {
                                       mod _assert_input_type {
                                           fn check_cs_arg_type<T>() where ::dubbo::serialize::SerdeJsonSerialization<T>: ::dubbo::serialize::Serializable, {}
                                           fn test_fn() {check_cs_arg_type<#arg_type>();}
                                       }
                                   });
                               },
                               Serialization::Protobuf => {
                                   assert_type_token_stream.extend(quote! {
                                       mod _assert_input_type {
                                           fn check_cs_arg_type<T>() where ::dubbo::serialize::ProstSerialization<T>: ::dubbo::serialize::Serializable, {}
                                           fn test_fn() {check_cs_arg_type<#arg_type>();}
                                       }
                                   });
                               },
                               Serialization::Other(_) => {
                                   assert_type_token_stream.extend(quote! {
                                       mod _assert_input_type {
                                           fn check_cs_arg_type<T>() where T: ::dubbo::serialize::Serializable, {}
                                           fn test_fn() {check_cs_arg_type<#arg_type>();}
                                       }
                                   });
                               }
                           }
                       }
                   }
                }
            }
        }
        
        if !contain_mut_self {
            return Err(syn::Error::new(input.span(), "rpc call must have mutable self receiver"));
        }
        
        
        Ok(assert_type_token_stream)
    }
        
    fn assert_output_args(output: &syn::ReturnType, rpc_call_meta_info: &RpcCallMetaInfo) -> syn::Result<TokenStream> {
        let mut assert_type_token_stream = TokenStream::default();

        let Some(deser) = rpc_call_meta_info.deserialization.clone() else {
            return Err(syn::Error::new(output.span(), "can not found deserialization configuration"));
        };
        
        match output {
            syn::ReturnType::Default => {
                return Err(syn::Error::new(output.span(), "rpc call must have return type"));
            },
            syn::ReturnType::Type(_, ty) => {
                match rpc_call_meta_info.rpc_type {
                    RpcType::Unary | RpcType::ClientStream => {
                        match &deser {
                            Deserialization::Json => {
                                assert_type_token_stream.extend(quote! {
                                    mod _assert_output_type {
                                        fn actual_return_type() -> #ty {unimplemented!()}
                                        fn return_type_checker<T, E>(actual_return_type: Result<T, E>) where ::dubbo::serialize::SerdeJsonDeserialization<T>: ::dubbo::serialize::Deserializable, E: From<Box<dyn ::std::error::Error + Send + Sync>>{}
                                        fn test_fn() {return_type_checker(actual_return_type());}
                                    }
                                    
                                });
                            },
                            Deserialization::Protobuf => {
                                assert_type_token_stream.extend(quote! {
                                    mod _assert_output_type {
                                        fn actual_return_type() -> #ty {unimplemented!()}
                                        fn return_type_checker<T, E>(actual_return_type: Result<T, E>) where ::dubbo::serialize::ProstDeserialization<T>: ::dubbo::serialize::Deserializable, E: From<Box<dyn ::std::error::Error + Send + Sync>>{}
                                        fn test_fn() {return_type_checker(actual_return_type());}
                                    }
                                    
                                });
                            },
                            Deserialization::Other(_) => {
                                assert_type_token_stream.extend(quote! {
                                    mod _assert_output_type {
                                        fn actual_return_type() -> #ty {unimplemented!()}
                                        fn return_type_checker<T, E>(actual_return_type: Result<T, E>) where T: ::dubbo::serialize::Deserializable, E: From<Box<dyn ::std::error::Error + Send + Sync>>{}
                                        fn test_fn() {return_type_checker(actual_return_type());}
                                    }
                                    
                                });
                            }
                        }                   
                    },
                    RpcType::ServerStream | RpcType::BiStream => {
                        match &deser {
                            Deserialization::Json => {
                                assert_type_token_stream.extend(quote! {
                                    mod _assert_output_type {
                                        fn actual_return_type() -> #ty {unimplemented!()}
                                        fn return_type_checker<T, E>(actual_return_type: Result<T, E>) where T: ::dubbo::serialize::Deserializable, E: From<Box<dyn ::std::error::Error + Send + Sync>>{}
                                        fn test_fn() {return_type_checker(actual_return_type());}
                                    }
                                    
                                });
                            },
                            Deserialization::Protobuf => {

                            },
                            Deserialization::Other(_) => {

                            }
                        }
                    }
                }
            }
        }
        
        
        
        
        Ok(assert_type_token_stream)
    }    
}


pub struct RpcCallMetaInfo {
    rpc_type: RpcType,
    serialization: Option<Serialization>,
    deserialization: Option<Deserialization>,
}

impl RpcCallMetaInfo {
    
    pub fn rpc_type(&self) -> RpcType {
        self.rpc_type
    }
    
    pub fn serialization(&self) -> Serialization {
        self.serialization.clone().unwrap_or(Serialization::Json)
    }
    
    pub fn deserialization(&self) -> Deserialization {
        self.deserialization.clone().unwrap_or(Deserialization::Json)
    }
}


impl syn::parse::Parse for RpcCallMetaInfo {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut rpc_type = RpcType::Unary;
        let mut serialization = None;
        let mut deserialization = None;
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
                    serialization = Some(Serialization::from(value));
                }
                "deser" => {
                    deserialization = Some(Deserialization::from(value));
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


#[derive(Clone, Copy)]
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