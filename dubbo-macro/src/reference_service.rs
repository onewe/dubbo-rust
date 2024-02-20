use std::{cell::OnceCell, collections::HashMap, sync::{atomic::AtomicUsize, OnceLock}};

use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{format_ident, quote, TokenStreamExt};
use syn::{punctuated::Punctuated, token::{Comma, Trait}, FnArg, Generics, Ident, LitStr, ReturnType, Signature, Token, TraitItemFn, Type};
use syn::Attribute;
use syn::parse_quote;


pub struct ReferenceService {
    attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
    ident: syn::Ident,
    items: Vec<syn::TraitItem>,
}

pub struct ReferenceMetaInfo {
    map: HashMap<String, String>,
}

impl ReferenceMetaInfo {

    const INTERFACE_NAME: &'static str = "interface";

    const SERIALIZATION: &'static str = "serialization";

    const DEFAULT_SERIALIZATION: &'static str = "json";

    const REFERENCE_TRAIT_NAME: &'static str = "reference_trait_name";


    pub fn get_interface_name(&self) -> String {
        self.map.get(ReferenceMetaInfo::INTERFACE_NAME).and_then(|t|Some(t.clone())).unwrap_or(self.get_reference_trait_name())
    }

    pub fn get_serialization(&self) -> String {
        self.map.get(ReferenceMetaInfo::SERIALIZATION).and_then(|t|Some(t.clone())).unwrap_or(ReferenceMetaInfo::DEFAULT_SERIALIZATION.to_owned())
    }

    pub fn set_reference_trait_name(&mut self, trait_name: String) {
        self.map.insert(ReferenceMetaInfo::REFERENCE_TRAIT_NAME.to_owned(), trait_name);
    }

    pub fn get_reference_trait_name(&self) -> String {
        self.map.get(ReferenceMetaInfo::REFERENCE_TRAIT_NAME).map(|t|t.clone()).unwrap_or_default()
    }

    pub fn new(attrs: Vec<MetaNameValue>) -> Self {
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

        ReferenceMetaInfo { map }
    }
}

impl syn::parse::Parse for ReferenceMetaInfo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs =
            syn::punctuated::Punctuated::<syn::MetaNameValue, Token![,]>::parse_terminated(input)?;
        Ok(ReferenceMetaInfo::new(attrs))
    }
}


impl ReferenceMetaInfo {

    fn to_token_stream(&self) -> TokenStream {
        let mut token_stream = TokenStream::new();

        let mut kv_token_streams = Vec::new();

        for (key, value) in &self.map {
            let key = key.as_str();
            let value = value.as_str();
            let kv_token_stream = quote! {
                #key = #value
            };
            
            kv_token_streams.push(kv_token_stream);
        }

        token_stream.extend(quote! {
            #[::dubbo_macro::reference_meta_info(#(#kv_token_streams),*)]
        });

        token_stream

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
    pub fn to_token_stream(self, mut reference_attr: ReferenceMetaInfo) -> syn::Result<TokenStream> {
        let ReferenceService {
            attrs,
            vis,
            ident,
            items,
        } = self;

        let reference_trait_name = ident.to_string();
        reference_attr.set_reference_trait_name(reference_trait_name);

        let functions_token_stream = ReferenceService::gen_proxy_method(&items, &reference_attr);

        let token_stream = quote::quote! {

           #(#attrs)*
           #vis struct #ident {
                invoker: ::dubbo::invoker::cloneable_invoker::CloneableInvoker,
           }

           impl #ident {
               #functions_token_stream
           }
        };

        Ok(token_stream)
    }

    fn gen_proxy_method(items: &Vec<syn::TraitItem>, reference_meta_info: &ReferenceMetaInfo) -> TokenStream {
        let mut token_stream = TokenStream::new();

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

                    if !ReferenceService::assert_include_mut_self_receiver(inputs) {
                        continue;
                    }


                    // let ReturnType::Type(_, return_ty) = output else {
                    //     continue;
                    // };
                    
                    // let assert_return_type = ReferenceService::gen_assert_return_type(serialization.as_str(), return_ty);

                    // let rpc_invocation = ReferenceService::gen_rpc_invocation(fn_ident.to_string(), inputs, interface_name.as_str(), serialization.as_str());
                    
                    // let rpc_response = ReferenceService::gen_grpc_response(serialization.as_str(), return_ty);

                    let (_, ty_generics, where_clause) = generics.split_for_impl();


                    let reference_meta_info_token_stream = reference_meta_info.to_token_stream();

                    let function_token_stream = quote! {
                        #(#attrs)*
                        #reference_meta_info_token_stream
                        pub fn #fn_ident #ty_generics(#inputs) #output #where_clause{

                            todo!()
                        }
                    };
                    token_stream.extend(function_token_stream);
                }
                _ => {}
            }
        }

        token_stream
    }

    fn assert_include_mut_self_receiver(input_args: &Punctuated<FnArg, Comma>,) -> bool {
        for arg in input_args {
            match arg {
                FnArg::Receiver(receiver) => {
                    if receiver.mutability.is_some() {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
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

                    let tt = ReferenceService::gen_grpc_argument(arg_name, serialization);
                    token_stream.extend(tt);
                }
                _ => {}
            }
        }
        
        token_stream.extend(quote! {
            let invocation = ::dubbo::invoker::RpcInvocation::new(interface_name.to_string(), method_name.to_string(), args);
        });
        
        token_stream
    }

    fn gen_grpc_argument(arg_name: Ident, serialization: &str) -> TokenStream {

        let arg_name_str = arg_name.to_string();

        if serialization == "json" {
            quote! {
                let #arg_name = ::dubbo::invoker::Argument::new(#arg_name_str.to_string(), Box::new(::dubbo::serialize::SerdeJsonSerialization::new(#arg_name)));
                args.push(#arg_name);
            }
        }
        else if serialization == "prost" {
            quote! {
                let #arg_name = ::dubbo::invoker::Argument::new(#arg_name_str.to_string(), Box::new(::dubbo::serialize::ProstSerialization::new(#arg_name)));
                args.push(#arg_name);
            }
        }
         else {
            quote! {
                let #arg_name = ::dubbo::invoker::Argument::new(#arg_name_str.to_string(), Box::new(#arg_name));
                args.push(#arg_name);
            }
        }
    }

    fn gen_grpc_response(serialization: &str, return_ty: &Type) -> TokenStream {
        if serialization == "json" {
            quote! {

                macro_rules! extract_ret_type {
                    (Result<$ret_type:ty,$ret_error_type:ty>) => {
                        $ret_type
                    };
                
                    (std::result::Result<$ret_type:ty,$ret_error_type:ty>) => {
                        $ret_type
                    };
                }


                let des: ::dubbo::serialize::SerdeJsonDeserialization<extract_ret_type![#return_ty]> = ::dubbo::serialize::SerdeJsonDeserialization::<extract_ret_type![#return_ty]>::new();
                let mut deserialize_data = ::dubbo::serialize::Deserializable::deserialize(&des, body)?;

                mod _check_return_type {
                    trait IsStreamData { const IS_STREAM: bool = false; }

                    impl<T: ?Sized> IsStreamData for T {}

                    struct Wrapper<T: ?Sized>(::std::marker::PhantomData<T>);

                    impl<T: ?Sized + ::futures::Stream> Wrapper<T> { const IS_STREAM: bool = true; }

                    pub(in super) fn check<T: ?Sized>() -> bool { Wrapper::<T>::IS_STREAM }
                }

                let is_stream_type = _check_return_type::check::<extract_ret_type![#return_ty]>();

                // if is_stream_type {
                //     deserialize_data
                // } else {
                //     ::futures::pin_mut!(deserialize_data);
                //     ::futures::StreamExt::next(deserialize_data).await
                // }

                // ::futures::pin_mut!(deserialize_data);
                // ::futures::StreamExt::next(deserialize_data).await

               
            }
        } else if serialization == "prost" {
            quote! {
                macro_rules! extract_ret_type {
                    (Result<$ret_type:ty,$ret_error_type:ty>) => {
                        $ret_type
                    };
                
                    (std::result::Result<$ret_type:ty,$ret_error_type:ty>) => {
                        $ret_type
                    };
                }

                let des: ::dubbo::serialize::ProstDeserialization<extract_ret_type![#return_ty]> = ::dubbo::serialize::ProstDeserialization::<extract_ret_type![#return_ty]>::new();
                let mut deserialize_data = ::dubbo::serialize::Deserializable::deserialize(&des, body);

                mod _check_return_type {
                    trait IsStreamData { const IS_STREAM: bool = false; }

                    impl<T: ?Sized> IsStreamData for T {}

                    struct Wrapper<T: ?Sized>(::std::marker::PhantomData<T>);

                    impl<T: ?Sized + ::futures::Stream> Wrapper<T> { const IS_STREAM: bool = true; }

                    pub(in super) fn check<T: ?Sized>() -> bool { Wrapper::<T>::IS_STREAM }
                }

                let is_stream_type = _check_return_type::check::<extract_ret_type![#return_ty]>();

                if is_stream_type {
                    deserialize_data
                } else {
                    ::futures::pin_mut!(deserialize_data);
                    ::futures::StreamExt::next(deserialize_data).await
                }
            }
        }
         else {
            quote! {
                macro_rules! extract_ret_type {
                    (Result<$ret_type:ty,$ret_error_type:ty>) => {
                        $ret_type
                    };
                
                    (std::result::Result<$ret_type:ty,$ret_error_type:ty>) => {
                        $ret_type
                    };
                }

                let des: extract_ret_type![#return_ty] = extract_ret_type![#return_ty]::default();
  
                let mut deserialize_data = ::dubbo::serialize::Deserializable::deserialize(&des, body);

                mod _check_return_type {
                    trait IsStreamData { const IS_STREAM: bool = false; }

                    impl<T: ?Sized> IsStreamData for T {}

                    struct Wrapper<T: ?Sized>(::std::marker::PhantomData<T>);

                    impl<T: ?Sized + ::futures::Stream> Wrapper<T> { const IS_STREAM: bool = true; }

                    pub(in super) fn check<T: ?Sized>() -> bool { Wrapper::<T>::IS_STREAM }
                }

                let is_stream_type = _check_return_type::check::<extract_ret_type![#return_ty]>();

                if is_stream_type {
                    deserialize_data
                } else {
                    ::futures::pin_mut!(deserialize_data);
                    ::futures::StreamExt::next(deserialize_data).await
                }
            }
        }
    }


    fn gen_assert_return_type(serialization: &str, rt: &Type) -> TokenStream {
        if serialization == "json" {
            quote! {
                mod _assert_return_type {
                    fn expect_return_type<T>(result: Result<T, ::dubbo::StdError>) where ::dubbo::serialize::SerdeJsonDeserialization<T>: ::dubbo::serialize::Deserializable {}
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
                    fn expect_return_type<T>(result: Result<T, ::dubbo::StdError>) where ::dubbo::serialize::ProstDeserialization<T>: ::dubbo::serialize::Deserializable {}
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

            quote! {
                mod _assert_return_type {
                    fn expect_return_type<T>(result: Result<T, ::dubbo::StdError>) where T: ::dubbo::serialize::Deserializable + std::default::Default {}
                    fn actual_return_type() -> #rt {
                        unimplemented!()
                    }

                    fn check_return_type(){
                        expect_return_type(actual_return_type())
                    }

                }
            }
        }
    }

}
