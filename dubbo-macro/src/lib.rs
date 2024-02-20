use proc_macro::TokenStream;

mod reference_service;
mod rpc_call;
mod reference_meta_info;



#[proc_macro_attribute]
pub fn reference(attr: TokenStream, input: TokenStream) -> TokenStream {

    let attr = syn::parse_macro_input!(attr as reference_service::ReferenceMetaInfo);
    let input = syn::parse_macro_input!(input as reference_service::ReferenceService);
    input.to_token_stream(attr).unwrap_or_else(|e|e.into_compile_error()).into()
}


#[proc_macro_attribute]
pub fn rpc_call(attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn reference_meta_info(attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}
