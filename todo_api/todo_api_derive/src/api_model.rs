use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub(crate) fn api_model_derive_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let result_name = syn::Ident::new(&format!("{}Result", name), name.span());

    let fields = if let syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(ref fields), .. }) = input.data {
        &fields.named
    } else {
        panic!("ApiModel only supports structs with named fields");
    };

    let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    let expanded = quote! {
        #[derive(Serialize, Deserialize, Debug)]
        pub struct #result_name {
            pub id: ::surrealdb::sql::Thing,
            #(pub #field_names: #field_types,)*
        }

        impl From<#result_name> for #name {
            fn from(value: #result_name) -> Self {
                Self {
                    #(#field_names: value.#field_names,)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}