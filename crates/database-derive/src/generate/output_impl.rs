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

        let relationship_fns = self.0.fields().filter_map(|field| {
			let relationship = field.relationship()?;
			let field_ident = field.ident();
			let relationship_name = relationship.name();
			let relationship_target = relationship.target();

			if field.is_optional() {
				return Some(quote! {
					async fn #relationship_name(
						&self,
						ctx: &::async_graphql::Context<'_>,
					) -> ::async_graphql::Result<::std::option::Option<::acebau_database::Record<#relationship_target>>> {
						use ::acebau_database::Repository as _;

						let ::std::option::Option::Some(id) = self.#field_ident else {
							return Ok(::std::option::Option::None);
						};
						let mut database = ctx.data::<::acebau_database::Database>()?.clone();

						Ok(::std::option::Option::Some(database
							.repository::<#relationship_target>()
							.fetch_by_id(id)
							.await?))
					}
				});
			}

			Some(quote! {
				async fn #relationship_name(
					&self,
					ctx: &::async_graphql::Context<'_>,
				) -> ::async_graphql::Result<::acebau_database::Record<#relationship_target>> {
					use ::acebau_database::Repository as _;

					let mut database = ctx.data::<::acebau_database::Database>()?.clone();

					Ok(database
						.repository::<#relationship_target>()
						.fetch_by_id(self.#field_ident)
						.await?)
				}
			})
		});

        let resolvers = self.0.resolvers().map(|resolver| {
            let name = resolver.name();
            let method = resolver.method();
            let ty = resolver.ty();

            quote! {
                async fn #name(
                    &self,
                    ctx: &::async_graphql::Context<'_>,
                ) -> ::async_graphql::Result<#ty> {
                    self.#method(ctx).await
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
            impl #ident {
                #(#getter_fns)*
                #(#relationship_fns)*
                #(#resolvers)*
            }
        });
    }
}
