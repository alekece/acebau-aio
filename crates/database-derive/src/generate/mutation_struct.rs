use crate::table::Table;
use derive_more::From;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};

#[derive(From)]
pub struct MutationStruct<'a>(&'a Table);

impl ToTokens for MutationStruct<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = self.0.ident();
        let input_ident = self.0.input_ident();
        let changeset_ident = self.0.changeset_ident();
        let mutation_ident = self.0.mutation_ident();
        let name = self.0.name();
        let create_fn = format_ident!("create_{name}");
        let update_fn = format_ident!("update_{name}");
        let patch_fn = format_ident!("patch_{name}");
        let delete_fn = format_ident!("delete_{name}");

        tokens.extend(quote! {
            #[derive(Default)]
            pub struct #mutation_ident;

            #[::async_graphql::Object]
            impl #mutation_ident {
                async fn #create_fn(
                    &self,
                    ctx: &::async_graphql::Context<'_>,
                    input: #input_ident,
                ) -> ::async_graphql::Result<::acebau_database::Record<#ident>> {
                    use ::acebau_database::Repository as _;

                    let item: #ident = input.into();
                    let mut database = ctx.data::<::acebau_database::Database>()?.clone();

                    Ok(database.repository::<#ident>()
                        .insert(&item).await?)
                }

                async fn #update_fn(
                    &self,
                    ctx: &::async_graphql::Context<'_>,
                    id: String,
                    input: #input_ident,
                ) -> ::async_graphql::Result<::acebau_database::Record<#ident>> {
                    use ::acebau_database::Repository as _;

                    let item: #ident = input.into();
                    let mut database = ctx.data::<::acebau_database::Database>()?.clone();

                    Ok(database.repository::<#ident>()
                        .update(::uuid::Uuid::parse_str(&id)?, &item).await?)
                }

                async fn #patch_fn(
                    &self,
                    ctx: &::async_graphql::Context<'_>,
                    id: String,
                    input: #changeset_ident,
                ) -> ::async_graphql::Result<::acebau_database::Record<#ident>> {
                    use ::acebau_database::Repository as _;

                    let mut database = ctx.data::<::acebau_database::Database>()?.clone();

                    Ok(database.repository::<#ident>()
                        .patch(::uuid::Uuid::parse_str(&id)?, &input).await?)
                }

                async fn #delete_fn(
                    &self,
                    ctx: &::async_graphql::Context<'_>,
                    id: String,
                ) -> ::async_graphql::Result<bool> {
                    use ::acebau_database::Repository as _;

                    let mut database = ctx.data::<::acebau_database::Database>()?.clone();

                    database.repository::<#ident>()
                        .delete(::uuid::Uuid::parse_str(&id)?).await?;
                    Ok(true)
                }
            }
        });
    }
}
