extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Meta, Ident};

#[proc_macro_derive(MyErrorTrait, attributes(success))]
pub fn my_error_trait_derive(tokens: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(tokens).unwrap();
    let name = &ast.ident;

    // find the #[success] in front of the enum variant
    let success_variant: Option<Ident> = if let syn::Data::Enum(data_enum) = &ast.data {
        data_enum
            .variants
            .iter()
            .filter_map(|variant| {
                variant
                    .attrs
                    .iter()
                    .find(|attr| {
                        if let Meta::Path(path) = &attr.meta {
                            path.is_ident("success")
                        } else {
                            panic!("Attribute must be a path and nothing else, e.g. #[success]");
                        }
                    })
                    .map(|_| variant.ident.clone())
            })
            .next()
    } else {
        None
    };

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
