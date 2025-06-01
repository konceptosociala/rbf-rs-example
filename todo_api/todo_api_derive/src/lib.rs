use api_model::api_model_derive_impl;
use proc_macro::TokenStream;

mod api_model;

#[proc_macro_derive(ApiModel)]
pub fn api_model_derive(input: TokenStream) -> TokenStream {
    api_model_derive_impl(input)
}