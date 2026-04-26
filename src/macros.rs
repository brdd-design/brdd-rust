// This would typically live in a separate crate with proc-macro = true

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, ItemImpl};

#[proc_macro_attribute]
pub fn brdd_use_case(attr: TokenStream, item: TokenStream) -> TokenStream {
    let id = attr.to_string();
    let input = parse_macro_input!(item as ItemImpl);
    
    // Logic to inject the use_case_id() method into the impl block
    let expanded = quote! {
        #input
        
        impl #input {
            pub fn get_use_case_id(&self) -> &str {
                #id
            }
        }
    };
    
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn brdd_rule(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    
    // Logic to tag the function for later discovery
    let expanded = quote! {
        #input
    };
    
    TokenStream::from(expanded)
}
