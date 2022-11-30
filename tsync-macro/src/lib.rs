extern crate proc_macro;

use proc_macro::TokenStream;
use quote::__private::TokenStream as TokenStream2;

// document this attribute
#[proc_macro_attribute]
pub fn tsync(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item: TokenStream2 = item.into();
    quote::quote! {
        /// tsync managed
        #item
    }
    .into()
}
