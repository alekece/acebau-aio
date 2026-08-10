use derive_more::From;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::table::Table;

#[derive(From)]
pub struct OutputImpl<'a>(&'a Table);

impl ToTokens for OutputImpl<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = self.0.ident();

        let getter_fns = self.0.fields().map(|field| {
            let field_ident = field.ident();
            let field_ty = field.ty();

            quote! {
                async fn #field_ident<'a>(&'a self) -> &'a #field_ty {
                    &self.#field_ident
                }
            }
        });

        tokens.extend(quote! {
            impl ::async_graphql::TypeName for #ident {
                fn type_name() -> ::std::borrow::Cow<'static, str> {
                    stringify!(#ident).into()
                }
            }

            #[::async_graphql::Object]
            impl #ident { #(#getter_fns)* }
        });
    }
}
