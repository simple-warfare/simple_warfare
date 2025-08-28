use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};
pub fn derive_encode(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let struct_name = &ast.ident;

    quote! {
        impl<'de> simple_warfare_shared::adaptor::message::MessageEncode<'de> for #struct_name{}
    }
    .into()
}

pub fn derive_decode(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let struct_name = &ast.ident;

    quote! {
        impl simple_warfare_shared::adaptor::message::MessageDecode for #struct_name{}
    }
    .into()
}
