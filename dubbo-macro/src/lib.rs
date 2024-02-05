use proc_macro::TokenStream;

mod reference_service;

#[proc_macro_attribute]
pub fn reference(attr: TokenStream, input: TokenStream) -> TokenStream {

    println!("attr: {:?}", attr);
    let input = syn::parse_macro_input!(input as reference_service::ReferenceService);
    input.to_token_stream().unwrap_or_else(|e|e.into_compile_error()).into()
}