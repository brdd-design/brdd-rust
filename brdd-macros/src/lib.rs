use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, ItemImpl, LitStr};

#[proc_macro_attribute]
pub fn brdd_use_case(attr: TokenStream, item: TokenStream) -> TokenStream {
    let id = parse_macro_input!(attr as LitStr);
    let input = parse_macro_input!(item as ItemImpl);
    let self_ty = &input.self_ty;

    let expanded = quote! {
        #input

        impl #self_ty {
            pub fn get_use_case_id(&self) -> &'static str {
                #id
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn brdd_rule(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    // Simple pass-through for now, can be extended for discovery
    let expanded = quote! {
        #input
    };
    TokenStream::from(expanded)
}
