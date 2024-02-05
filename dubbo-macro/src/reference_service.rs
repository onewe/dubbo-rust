use proc_macro2::{TokenStream, TokenTree};
use quote::{format_ident, quote, TokenStreamExt};
use syn::{token::Trait, Generics, Ident, Signature, Token, TraitItemFn};

pub struct ReferenceService {
    attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
    ident: syn::Ident,
    items: Vec<syn::TraitItem>,
}


pub struct ReferenceAttr {
    attrs: syn::punctuated::Punctuated<syn::ExprAssign, syn::token::Comma>,
}


impl syn::parse::Parse for ReferenceAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        
        let attrs: syn::punctuated::Punctuated<syn::ExprAssign, syn::token::Comma> = syn::punctuated::Punctuated::<syn::ExprAssign, Token![,]>::parse_terminated(input)?;
        println!("attrs: {:?}", attrs);
        Ok(ReferenceAttr {attrs})
    }
}

impl syn::parse::Parse for ReferenceService {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let item = input.parse::<syn::ItemTrait>()?;

        let attrs = item.attrs;
        let vis = item.vis;
        let ident = item.ident;
        let items = item.items.into_iter().filter(|item| {
            match item {
                syn::TraitItem::Fn(_) => true,
                _ => false,
            }
        }).collect();



        Ok(ReferenceService {
            attrs,
            vis,
            ident,
            items,
        })
    }
}


impl ReferenceService {
    pub fn to_token_stream(self) -> syn::Result<TokenStream> {

        let ReferenceService {
            attrs,
            vis,
            ident,
            items,
        } = self;


        let functions_token_stream = ReferenceService::gen_proxy_method(&items, &ident);
        
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


    fn gen_proxy_method(items: &Vec<syn::TraitItem>, trait_ident: &Ident) -> TokenStream {

        let mut token_stream = TokenStream::new();

        for item in items {
            match item {
                syn::TraitItem::Fn(function) => {
                    let TraitItemFn {
                        attrs,
                        sig,
                        ..
                    } = function;

                    let Signature {
                        ident: fn_ident,
                        generics,
                        inputs,
                        output,
                        ..
                    } = sig;

                    let (_, ty_generics, where_clause) = generics.split_for_impl();
                    
                    let function_token_stream = quote! {
                        #(#attrs)*
                        pub fn #fn_ident #ty_generics(#inputs) #output #where_clause{
                            "test"
                        }
                    };
                    token_stream.extend(function_token_stream);
                   
                },
                _ => {}
            }
        }
      
        token_stream
    }

    fn gen_rpc_invocation() -> TokenStream {
        
        todo!()
    }
}
