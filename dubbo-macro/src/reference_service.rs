use std::collections::HashMap;

use proc_macro2::{TokenStream, TokenTree};
use quote::{format_ident, quote, TokenStreamExt};
use syn::{punctuated::Punctuated, token::{Comma, Trait}, FnArg, Generics, Ident, ReturnType, Signature, Token, TraitItemFn, Type};

pub struct ReferenceService {
    attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
    ident: syn::Ident,
    items: Vec<syn::TraitItem>,
}

pub struct ReferenceAttr {
    map: HashMap<String, String>,
}

impl ReferenceAttr {

    const INTERFACE_NAME: &'static str = "interface";

    const SERIALIZATION: &'static str = "serialization";

    const DEFAULT_SERIALIZATION: &'static str = "json";


    fn get_interface_name(&self) -> Option<String> {
        self.map.get(ReferenceAttr::INTERFACE_NAME).and_then(|t|Some(t.clone()))
    }

    fn get_serialization(&self) -> String{
        self.map.get(ReferenceAttr::SERIALIZATION).and_then(|t|Some(t.clone())).unwrap_or(ReferenceAttr::DEFAULT_SERIALIZATION.to_owned())
    }

}

impl syn::parse::Parse for ReferenceAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs =
            syn::punctuated::Punctuated::<syn::MetaNameValue, Token![,]>::parse_terminated(input)?;

        let mut map = HashMap::new();

        for attr in attrs {
            let key = attr.path.get_ident();
            match key {
                Some(key) => match attr.value {
                    syn::Expr::Lit(lit) => match lit.lit {
                        syn::Lit::Str(str_lit) => {
                            let value = str_lit.value();
                            let key = key.to_string();
                            map.insert(key, value);
                        }
                        _ => {
                            continue;
                        }
                    },
                    _ => {
                        continue;
                    }
                },
                None => {
                    continue;
                }
            }
        }

        Ok(ReferenceAttr { map })
    }
}

impl syn::parse::Parse for ReferenceService {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let item = input.parse::<syn::ItemTrait>()?;

        let attrs = item.attrs;
        let vis = item.vis;
        let ident = item.ident;
        let items = item
            .items
            .into_iter()
            .filter(|item| match item {
                syn::TraitItem::Fn(_) => true,
                _ => false,
            })
            .collect();

        Ok(ReferenceService {
            attrs,
            vis,
            ident,
            items,
        })
    }
}

impl ReferenceService {
    pub fn to_token_stream(self, attr: ReferenceAttr) -> syn::Result<TokenStream> {
        let ReferenceService {
            attrs,
            vis,
            ident,
            items,
        } = self;

        let functions_token_stream = ReferenceService::gen_proxy_method(&items, &ident, &attr);

        let token_stream = quote::quote! {

           #(#attrs)*
           #vis struct #ident {
           }

           impl #ident {
               #functions_token_stream
           }
        };

        Ok(token_stream)
    }

    fn gen_proxy_method(items: &Vec<syn::TraitItem>, trait_ident: &Ident, attr: &ReferenceAttr) -> TokenStream {
        let mut token_stream = TokenStream::new();

        let interface_name = attr.get_interface_name().unwrap_or(trait_ident.to_string());
        let serialization = attr.get_serialization();


        for item in items {
            match item {
                syn::TraitItem::Fn(function) => {
                    let TraitItemFn { attrs, sig, .. } = function;

                    let Signature {
                        ident: fn_ident,
                        generics,
                        inputs,
                        output,
                        ..
                    } = sig;

                    let ReturnType::Type(_, return_ty) = output else {
                        continue;
                    };
                    
                    let assert_return_type = ReferenceService::gen_assert_return_type(serialization.as_str(), return_ty);

                    let rpc_invocation = ReferenceService::gen_rpc_invocation(fn_ident.to_string(), inputs, interface_name.as_str(), serialization.as_str());
                    

                    let (_, ty_generics, where_clause) = generics.split_for_impl();

                    let function_token_stream = quote! {
                        #(#attrs)*
                        pub async fn #fn_ident #ty_generics(#inputs) #output #where_clause{
                            #assert_return_type
                            #rpc_invocation
                        }
                    };
                    token_stream.extend(function_token_stream);
                }
                _ => {}
            }
        }

        token_stream
    }

    fn gen_rpc_invocation(method_name: String, input_args: &Punctuated<FnArg, Comma>, interface_name: &str, serialization: &str) -> TokenStream {
        
        let mut token_stream = quote! {
            let interface_name = #interface_name;
            let method_name = #method_name;
            let mut args = Vec::new();
        };

        for arg in input_args {
            match arg {
                FnArg::Typed(pat_type) => {
                    let arg_name = match *pat_type.pat {
                        syn::Pat::Ident(ref ident) => {
                            ident.ident.clone()
                        },
                        _ => {
                            continue;
                        }
                    };

                    let arg_name_str = arg_name.to_string();

                    let tt = ReferenceService::gen_grpc_argument(arg_name_str, arg_name, serialization);
                    token_stream.extend(tt);
                }
                _ => {}
            }
        }
        
        token_stream.extend(quote! {
            let invocation = dubbo::invoker::RpcInvocation::new(interface_name.to_string(), method_name.to_string(), args);
        });
        
        token_stream
    }

    fn gen_grpc_argument(arg_name_str:String, arg_name: Ident, serialization: &str) -> TokenStream {

        if serialization == "json" {
            quote! {
                let #arg_name = dubbo::invoker::Argument::new(#arg_name_str, Box::new(dubbo::serialize::SerdeJsonSerialization::new(#arg_name)));
                args.push(#arg_name);
            }
        }
        else if serialization == "prost" {
            quote! {
                let #arg_name = dubbo::invoker::Argument::new(#arg_name_str, Box::new(dubbo::serialize::ProstSerialization::new(#arg_name)));
                args.push(#arg_name);
            }
        }
         else {
            TokenStream::new()
        }
    }


    fn gen_assert_return_type(serialization: &str, rt: &Type) -> TokenStream {
        if serialization == "json" {
            quote! {
                mod _assert_return_type {
                    fn expect_return_type<T>(result: Result<T, dubbo::StdError>) where dubbo::serialize::SerdeJsonDeserialization<T>: dubbo::serialize::Deserializable {}
                    fn actual_return_type() -> #rt {
                        unimplemented!()
                    }

                    fn check_return_type(){
                        expect_return_type(actual_return_type())
                    }

                }
            }
        } else if serialization == "prost" {
            quote! {
                mod _assert_return_type {
                    fn expect_return_type<T>(result: Result<T, dubbo::StdError>) where dubbo::serialize::ProstDeserialization<T>: dubbo::serialize::Deserializable {}
                    fn actual_return_type() -> #rt {
                        unimplemented!()
                    }

                    fn check_return_type(){
                        expect_return_type(actual_return_type())
                    }

                }
            }
        }
         else {
            TokenStream::new()
        }
    }

}
