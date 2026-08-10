use derive_more::From;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::table::Table;

#[derive(From)]
pub struct InputStruct<'a>(&'a Table);

impl ToTokens for InputStruct<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = self.0.ident();
        let input_ident = self.0.input_ident();
        let input_name = input_ident.to_string();

        let (field_declrs, field_assigns) = self
            .0
            .fields()
            .map(|field| {
                let field_ident = field.ident();
                let field_ty = field.ty();

                (
                    quote! { pub #field_ident: #field_ty },
                    quote! { #field_ident: value.#field_ident },
                )
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();

        tokens.extend(quote! {
            #[derive(Debug, Clone, ::async_graphql::InputObject)]
            #[graphql(name = #input_name)]
            pub struct #input_ident {
                #(#field_declrs),*
            }

            impl ::std::convert::From<#input_ident> for #ident {
                fn from(value: #input_ident) -> Self {
                    Self { #(#field_assigns),* }
                }
            }
        });
    }
}
