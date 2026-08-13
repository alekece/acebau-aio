use derive_more::From;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};

use crate::table::Table;

#[derive(From)]
pub struct QueryStruct<'a>(&'a Table);

impl ToTokens for QueryStruct<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = self.0.ident();
        let query_ident = self.0.query_ident();
        let getter_fn = format_ident!("{}", self.0.plural_name());
        let getter_by_id_fn = format_ident!("{}", self.0.name());

        tokens.extend(quote! {
            #[derive(Default)]
            pub struct #query_ident;

            #[::async_graphql::Object]
            impl #query_ident {
                async fn #getter_fn(
                    &self,
                    ctx: &::async_graphql::Context<'_>,
                    page: ::std::option::Option<i32>,
                    page_size: ::std::option::Option<i32>,
                ) -> ::async_graphql::Result<::acebau_database::Page<#ident>> {
                    use ::acebau_database::Repository as _;

                    let mut database = ctx.data::<::acebau_database::Database>()?.clone();
                    let page = page.unwrap_or(1);
                    let page_size = page_size.unwrap_or(10);
                    let page_number = u32::try_from(page)
                        .map_err(|_| "page number must be greater than zero")?;
                    let size = u32::try_from(page_size)
                        .map_err(|_| "page size must be positive")?;
                    let options = ::acebau_database::FetchOptions::default()
                        .with_page(page_number, size)?;
                    let items = database
                        .repository::<#ident>()
                        .fetch_all(options.clone())
                        .await?;
                    let total_items = database
                        .repository::<#ident>()
                        .count(options)
                        .await?;
                    let total_pages = total_items.div_ceil(u64::from(size));

                    Ok(::acebau_database::Page {
                        items,
                        page,
                        page_size,
                        total_items: i32::try_from(total_items)
                            .map_err(|_| "record count exceeds the GraphQL integer range")?,
                        total_pages: i32::try_from(total_pages)
                            .map_err(|_| "page count exceeds the GraphQL integer range")?,
                    })
                }

                async fn #getter_by_id_fn(
                    &self,
                    ctx: &::async_graphql::Context<'_>,
                    id: String,
                ) -> ::async_graphql::Result<::acebau_database::Record<#ident>> {
                    use ::acebau_database::Repository as _;

                    let mut database = ctx.data::<::acebau_database::Database>()?.clone();

                    Ok(database
                        .repository::<#ident>()
                        .fetch_by_id(::uuid::Uuid::parse_str(&id)?)
                        .await?)
                }
            }
        });
    }
}
