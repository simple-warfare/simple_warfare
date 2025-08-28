use proc_macro::TokenStream;
mod adaptor;

#[proc_macro_derive(MessageEncode)]
pub fn derive_message_encode(input: TokenStream) -> TokenStream {
    adaptor::message::derive_encode(input)
}

#[proc_macro_derive(MessageDecode)]
pub fn derive_message_decode(input: TokenStream) -> TokenStream {
    adaptor::message::derive_decode(input)
}
