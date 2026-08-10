use derive_more::From;
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::table::Table;

#[derive(From)]
pub struct ChangesetStruct<'a>(&'a Table);

impl ToTokens for ChangesetStruct<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = self.0.changeset_ident();
        let fields = self.0.fields().filter(|field| !field.skip()).collect::<Vec<_>>();

        if fields.is_empty() {
            tokens.extend(quote! {
                #[derive(Debug, Default, Clone, ::async_graphql::InputObject)]
                pub struct #ident {
                    pub status: ::std::option::Option<::acebau_database::Status>,
                }

                impl #ident {
                    pub fn is_empty(&self) -> bool {
                        self.status.is_none()
                    }

                    pub fn with_status(mut self, value: ::acebau_database::Status) -> Self {
                        self.status = Some(value);
                        self
                    }
                }
            });

            return;
        }

        let (field_idents, field_declrs, setter_fns) = fields
            .iter()
            .map(|field| {
                let field_ident = field.ident();
                let field_ty = field.ty();
                let setter_ident = self.0.setter(field_ident);

                (
                    field_ident,
                    quote! {
                        pub #field_ident: ::std::option::Option<#field_ty>
                    },
                    quote! {
                        pub fn #setter_ident(mut self, value: impl Into<#field_ty>) -> Self {
                            self.#field_ident = Some(value.into());

                            self
                        }
                    },
                )
            })
            .multiunzip::<(Vec<_>, Vec<_>, Vec<_>)>();

        tokens.extend(quote! {
            #[derive(Debug, Clone, ::async_graphql::InputObject)]
            pub struct #ident {
                pub status: ::std::option::Option<::acebau_database::Status>,
                #(#field_declrs),*
            }

            impl Default for #ident {
                fn default() -> Self {
                    Self {
                        status: ::std::option::Option::None,
                        #(#field_idents: ::std::option::Option::None),*
                    }
                }
            }

            impl #ident {
                pub fn is_empty(&self) -> bool {
                    self.status.is_none() && #(self.#field_idents.is_none())&&*
                }

                pub fn with_status(mut self, value: ::acebau_database::Status) -> Self {
                    self.status = Some(value);
                    self
                }

                #(#setter_fns)*
            }
        });
    }
}
