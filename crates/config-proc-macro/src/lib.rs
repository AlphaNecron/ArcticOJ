use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_attribute]
pub fn model(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    if let Data::Struct(ref mut ds) = input.data {
        if let Fields::Named(ref mut fields) = ds.fields {
            for f in &mut fields.named {
                f.vis = input.vis.clone();
            }
        }
    }

    let derive = match input.data {
        Data::Struct(_) => quote!(#[derive(knus::Decode)]),
        Data::Enum(_) => quote!(#[derive(knus::DecodeScalar)]),
        _ => quote!(),
    };

    TokenStream::from(quote! {
        #[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
        #derive
        #input
    })
}
