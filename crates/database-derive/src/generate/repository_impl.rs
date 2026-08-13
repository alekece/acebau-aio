use derive_more::From;
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::table::Table;

#[derive(From)]
pub struct RepositoryImpl<'a>(&'a Table);

impl RepositoryImpl<'_> {
    fn generate_insert_fn(&self) -> TokenStream {
        let ident = self.0.ident();

        let (field_names, field_values, field_idents): (Vec<_>, Vec<_>, Vec<_>) = self
            .0
            .fields()
            .enumerate()
            .map(|(i, field)| (field.name(), format!("${}", i + 1), field.ident()))
            .multiunzip();

        let query = format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING *",
            self.0.name(),
            field_names.join(", "),
            field_values.join(", "),
        );

        quote! {
            async fn insert(
                &mut self,
                item: &#ident,
            ) -> ::std::result::Result<::acebau_database::Record<#ident>, Self::Error> {
                sqlx::query_as(#query)
                    #(.bind(&item.#field_idents))*
                    .fetch_one(<Self as ::acebau_database::Executor>::executor(self))
                    .await
                    .map_err(Self::Error::from)
            }
        }
    }

    fn generate_patch_fn(&self) -> TokenStream {
        let ident = self.0.ident();
        let (field_names, field_idents): (Vec<_>, Vec<_>) = self
            .0
            .fields()
            .filter(|field| !field.skip())
            .map(|field| (field.name(), field.ident()))
            .unzip();
        let start_query = format!("UPDATE {} SET ", self.0.name());

        quote! {
            async fn patch(
                &mut self,
                id: ::uuid::Uuid,
                changeset: &Self::Changeset,
            ) -> ::std::result::Result<::acebau_database::Record<#ident>, Self::Error> {
                if changeset.is_empty() {
                    return Err(Self::Error::NoChanges { id });
                }

                let mut query_builder = sqlx::QueryBuilder::new(#start_query);
                let mut values = query_builder.separated(", ");

                #(
                    if let Some(value) = &changeset.#field_idents {
                        values
                            .push(#field_names)
                            .push_unseparated(" = ")
                            .push_bind_unseparated(value);
                    }
                )*

                if let Some(status) = changeset.status {
                    values
                        .push("status = ")
                        .push_bind_unseparated(status);
                }

                query_builder
                    .push(" WHERE id = ")
                    .push_bind(id)
                    .push(" RETURNING *");

                query_builder
                    .build_query_as()
                    .fetch_one(<Self as ::acebau_database::Executor>::executor(self))
                    .await
                    .map_err(Self::Error::from)
            }
        }
    }

    fn generate_update_fn(&self) -> TokenStream {
        let ident = self.0.ident();
        let (field_names, field_values, field_idents): (Vec<_>, Vec<_>, Vec<_>) = self
            .0
            .fields()
            .filter(|field| !field.skip())
            .enumerate()
            .map(|(index, field)| (field.name(), format!("${}", index + 1), field.ident()))
            .multiunzip();
        let query = format!(
            "UPDATE {} SET {} WHERE id = ${} RETURNING *",
            self.0.name(),
            field_names
                .iter()
                .zip(&field_values)
                .map(|(name, value)| format!("{name} = {value}"))
                .join(", "),
            field_values.len() + 1,
        );

        quote! {
            async fn update(
                &mut self,
                id: ::uuid::Uuid,
                item: &#ident,
            ) -> ::std::result::Result<::acebau_database::Record<#ident>, Self::Error> {
                sqlx::query_as(#query)
                    #(.bind(&item.#field_idents))*
                    .bind(id)
                    .fetch_one(<Self as ::acebau_database::Executor>::executor(self))
                    .await
                    .map_err(Self::Error::from)
            }
        }
    }

    fn generate_fetch_by_id_fn(&self) -> TokenStream {
        let ident = self.0.ident();
        let query = format!("SELECT * FROM {} WHERE id = $1", self.0.name());

        quote! {
            async fn fetch_by_id(
                &mut self,
                id: ::uuid::Uuid,
            ) -> ::std::result::Result<::acebau_database::Record<#ident>, Self::Error> {
                sqlx::query_as(#query)
                    .bind(id)
                    .fetch_one(<Self as ::acebau_database::Executor>::executor(self))
                    .await
                    .map_err(|error| match error {
                        sqlx::Error::RowNotFound => Self::Error::NotFound { id },
                        _ => error.into(),
                    })
            }
        }
    }

    fn generate_fetch_all_fn(&self) -> TokenStream {
        let ident = self.0.ident();
        let query = format!("SELECT * FROM {}", self.0.name());

        quote! {
            async fn fetch_all(
                &mut self,
                options: ::acebau_database::FetchOptions,
            ) -> ::std::result::Result<Vec<::acebau_database::Record<#ident>>, Self::Error> {
                let mut query_builder = sqlx::QueryBuilder::new(#query);

                for (i, status) in options.statuses().enumerate() {
                    if i == 0 {
                        query_builder.push(" WHERE status = ").push_bind(status);
                    } else {
                        query_builder.push(" OR status = ").push_bind(status);
                    }
                }

                query_builder
                    .push(" ORDER BY created_at DESC LIMIT ")
                    .push_bind(i64::from(options.limit()))
                    .push(" OFFSET ")
                    .push_bind(i64::from(options.offset()));

                query_builder
                    .build_query_as()
                    .fetch_all(<Self as ::acebau_database::Executor>::executor(self))
                    .await
                    .map_err(Self::Error::from)
            }
        }
    }

    fn generate_count_fn(&self) -> TokenStream {
        let query = format!("SELECT COUNT(*) FROM {}", self.0.name());

        quote! {
            async fn count(
                &mut self,
                options: ::acebau_database::FetchOptions,
            ) -> ::std::result::Result<u64, Self::Error> {
                let mut query_builder = sqlx::QueryBuilder::new(#query);

                for (i, status) in options.statuses().enumerate() {
                    if i == 0 {
                        query_builder.push(" WHERE status = ").push_bind(status);
                    } else {
                        query_builder.push(" OR status = ").push_bind(status);
                    }
                }

                let count: i64 = query_builder
                    .build_query_scalar()
                    .fetch_one(<Self as ::acebau_database::Executor>::executor(self))
                    .await
                    .map_err(Self::Error::from)?;

                u64::try_from(count).map_err(|_| Self::Error::Internal {
                    source: sqlx::Error::Protocol("database returned a negative record count".into()),
                })
            }
        }
    }

    fn generate_delete_fn(&self) -> TokenStream {
        let query = format!("DELETE FROM {} WHERE id = $1", self.0.name());

        quote! {
            async fn delete(
                &mut self,
                id: ::uuid::Uuid,
            ) -> ::std::result::Result<(), Self::Error> {
                match sqlx::query(#query)
                    .bind(id)
                    .execute(<Self as ::acebau_database::Executor>::executor(self))
                    .await?
                    .rows_affected()
                {
                    0 => Err(::acebau_database::RepositoryError::NotFound { id }),
                    1 => Ok(()),
                    _ => unreachable!(),
                }
            }
        }
    }
}

impl ToTokens for RepositoryImpl<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = self.0.ident();
        let changeset_ident = self.0.changeset_ident();
        let insert_fn = self.generate_insert_fn();
        let patch_fn = self.generate_patch_fn();
        let update_fn = self.generate_update_fn();
        let fetch_by_id_fn = self.generate_fetch_by_id_fn();
        let fetch_all_fn = self.generate_fetch_all_fn();
        let count_fn = self.generate_count_fn();
        let delete_fn = self.generate_delete_fn();

        tokens.extend(quote! {
            impl<T> ::acebau_database::Repository<#ident> for ::acebau_database::DatabaseHandle<T>
            where
                Self: for<'a> ::acebau_database::Executor<'a>,
                T: ::std::marker::Send,
                #ident: ::std::marker::Sync + ::async_graphql::ObjectType + ::async_graphql::TypeName,
                #changeset_ident: ::std::marker::Sync,
            {
                type Error = ::acebau_database::RepositoryError;
                type Changeset = #changeset_ident;

                #insert_fn
                #patch_fn
                #update_fn
                #fetch_by_id_fn
                #fetch_all_fn
                #count_fn
                #delete_fn
            }
        });
    }
}
