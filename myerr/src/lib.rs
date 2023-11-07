extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, Meta, LitStr, meta::ParseNestedMeta};

#[proc_macro_derive(MyErrorTrait, attributes(success))]
pub fn my_error_trait_derive(tokens: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(tokens).unwrap();
    let name = &ast.ident;
    let mut success_variant : Option<Ident> = None;

    // find the #[success] in front of the enum variant
    if let syn::Data::Enum(data_enum) = &ast.data {
        for variant in &data_enum.variants {
            for attr in &variant.attrs {
                if attr.path().is_ident("success") {
                    success_variant = Some(variant.ident.clone());
                }
            }
        }
    }

    // Generate the implementation of the trait
    if let Some(success_variant) = success_variant {
        let expanded = quote! {
            impl #name {
                const SUCCESS: #name = #name::#success_variant;
            }

            impl<T> From<Result<T, #name>> for #name {
                fn from(ret: Result<T, #name>) -> Self {
                    match ret {
                        Ok(_) => #name::SUCCESS,
                        Err(err) => err,
                    }
                }
            }
        };

        TokenStream::from(expanded)
    } else {
        panic!("No success value defined, please add a #[success] attribute to your enum");
    }
}
