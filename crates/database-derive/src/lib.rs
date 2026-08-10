use darling::{
    FromDeriveInput, FromField, FromMeta,
    ast::{Data, Fields},
    util::Ignored,
};
use itertools::Itertools;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::{format_ident, quote};
use syn::{DeriveInput, Ident, Type, parse_macro_input};

#[derive(FromDeriveInput)]
#[darling(attributes(table), supports(struct_named))]
struct TableOptions {
    ident: Ident,
    name: String,
    data: Data<Ignored, FieldOptions>,
}

impl TableOptions {
    fn fields(&self) -> impl Iterator<Item = &FieldOptions> {
        let Data::Struct(Fields { fields, .. }) = &self.data else {
            unreachable!("Table can only be used with structs");
        };

        fields.iter()
    }
}

#[derive(FromField)]
#[darling(attributes(table))]
struct FieldOptions {
    ident: Option<Ident>,
    ty: Type,
    #[darling(default)]
    rename: Option<String>,
    #[darling(default)]
    skip: Option<bool>,
}

impl FieldOptions {
    fn is_type_option(&self) -> bool {
        if let Type::Path(type_path) = &self.ty
            && let Some(type_segment) = type_path.path.segments.last()
            && type_segment.ident == "Option"
        {
            true
        } else {
            false
        }
    }
}

#[derive(Default, FromDeriveInput)]
#[darling(default, attributes(changeset), supports(struct_named))]
struct ChangesetOptions {
    #[darling(default)]
    setter: Option<SetterOptions>,
}

impl ChangesetOptions {
    fn setter(&self, ident: &Ident) -> Ident {
        if let Some(prefix) = self.setter.as_ref().and_then(|setter| setter.prefix.as_deref()) {
            format_ident!("{prefix}_{ident}")
        } else {
            ident.clone()
        }
    }
}

#[derive(FromMeta)]
struct SetterOptions {
    #[darling(default)]
    prefix: Option<String>,
}

#[proc_macro_error]
#[proc_macro_derive(Table, attributes(table, changeset))]
pub fn derive_table(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let table_options = match TableOptions::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let changeset_options = match ChangesetOptions::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let ident = &table_options.ident;
    let changeset_ident = format_ident!("{}Changeset", ident);

    let changeset = generate_changeset(&changeset_ident, &changeset_options, &table_options);
    let insert = generate_insert(&table_options);
    let update = generate_update(&table_options);
    let fetch_by_id = generate_fetch_by_id(&table_options);
    let fetch_all = generate_fetch_all(&table_options);
    let delete = generate_delete(&table_options);
    let status = generate_status(&table_options);

    quote! {
        #changeset

        impl<T> crate::Repository<#ident> for crate::DatabaseHandle<T>
        where
            Self: for<'a> crate::Executor<'a>,
            T: ::std::marker::Send,
            #ident: ::std::marker::Sync,
            #changeset_ident: ::std::marker::Sync,
        {
            type Error = crate::RepositoryError;
            type Changeset = #changeset_ident;

            #insert
            #update
            #fetch_by_id
            #fetch_all
            #delete
            #status
        }
    }
    .into()
}

fn generate_insert(table_options: &TableOptions) -> proc_macro2::TokenStream {
    let ident = &table_options.ident;
    let (field_names, field_values, field_idents): (Vec<_>, Vec<_>, Vec<_>) = table_options
        .fields()
        .enumerate()
        .map(|(i, attr)| {
            (
                attr.rename.clone().unwrap_or(attr.ident.as_ref().unwrap().to_string()),
                format!("${}", i + 1),
                attr.ident.as_ref().unwrap(),
            )
        })
        .multiunzip();

    let query = format!(
        "INSERT INTO {} ({}, status) VALUES ({}, ${}) RETURNING *",
        table_options.name,
        field_names.join(", "),
        field_values.join(", "),
        field_values.len() + 1
    );

    quote! {
        async fn insert(&mut self, item: &#ident, status: crate::types::Status) -> ::std::result::Result<crate::Record<#ident>, Self::Error> {
            sqlx::query_as(#query)
                #(.bind(&item.#field_idents))*
                .bind(status)
                .fetch_one(<Self as crate::Executor>::executor(self))
                .await
                .map_err(Self::Error::from)
        }
    }
}

fn generate_update(table_options: &TableOptions) -> proc_macro2::TokenStream {
    let ident = &table_options.ident;
    let (field_names, field_idents): (Vec<_>, Vec<_>) = table_options
        .fields()
        .filter(|attr| !attr.skip.unwrap_or(false))
        .map(|attr| {
            (
                attr.rename.clone().unwrap_or(attr.ident.as_ref().unwrap().to_string()),
                attr.ident.as_ref().unwrap(),
            )
        })
        .unzip();

    let start_query = format!("UPDATE {} SET ", table_options.name);

    quote! {
        async fn update(
            &mut self,
            id: uuid::Uuid,
            changeset: &Self::Changeset,
            status: ::std::option::Option<crate::types::Status>,
        ) -> ::std::result::Result<crate::Record<#ident>, Self::Error> {
            if changeset.is_empty() && status.is_none() {
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

            if let Some(status) = status {
                values.push("status = ").push_bind_unseparated(status);
            }

            query_builder.push(" WHERE id = ").push_bind(id).push(" RETURNING *");

            query_builder
                .build_query_as()
                .fetch_one(<Self as crate::Executor>::executor(self))
                .await
                .map_err(Self::Error::from)
        }
    }
}

fn generate_fetch_by_id(table_options: &TableOptions) -> proc_macro2::TokenStream {
    let ident = &table_options.ident;
    let query = format!("SELECT * FROM {} WHERE id = $1", table_options.name);

    quote! {
        async fn fetch_by_id(&mut self, id: uuid::Uuid) -> ::std::result::Result<crate::Record<#ident>, Self::Error> {
            sqlx::query_as(#query)
                .bind(id)
                .fetch_one(<Self as crate::Executor>::executor(self))
                .await
                .map_err(|error| match error {
                    sqlx::Error::RowNotFound => Self::Error::NotFound { id },
                    _ => error.into(),
                })
        }
    }
}

fn generate_fetch_all(table_options: &TableOptions) -> proc_macro2::TokenStream {
    let ident = &table_options.ident;
    let start_query = format!("SELECT * FROM {}", table_options.name);

    quote! {
        async fn fetch_all(&mut self, options: crate::FetchOptions) -> ::std::result::Result<Vec<crate::Record<#ident>>, Self::Error> {
            let mut query_builder = sqlx::QueryBuilder::new(#start_query);

            for (i, status) in options.statuses().enumerate() {
                if i == 0 {
                    query_builder.push(" WHERE status = ").push_bind(status);
                } else {
                    query_builder.push(" OR status = ").push_bind(status);
                }
            }

            query_builder
                .build_query_as()
                .fetch_all(<Self as crate::Executor>::executor(self))
                .await
                .map_err(Self::Error::from)
        }
    }
}

fn generate_delete(table_options: &TableOptions) -> proc_macro2::TokenStream {
    let query = format!("DELETE FROM {} WHERE id = $1", table_options.name);

    quote! {
        async fn delete(&mut self, id: uuid::Uuid) -> ::std::result::Result<(), Self::Error> {
            match sqlx::query(#query)
                .bind(id)
                .execute(<Self as crate::Executor>::executor(self))
                .await?
                .rows_affected()
            {
                0 => Err(crate::RepositoryError::NotFound { id }),
                1 => Ok(()),
                _ => unreachable!(),
            }
        }
    }
}

fn generate_status(table_options: &TableOptions) -> proc_macro2::TokenStream {
    let query = format!("SELECT status FROM {} WHERE id = $1", table_options.name);

    quote! {
        async fn status(&mut self, id: uuid::Uuid) -> ::std::result::Result<crate::types::Status, Self::Error> {
            sqlx::query_scalar(#query)
                .bind(id)
                .fetch_one(<Self as crate::Executor>::executor(self))
                .await
                .map_err(|error| match error {
                    sqlx::Error::RowNotFound => Self::Error::NotFound { id },
                    _ => error.into(),
                })
        }
    }
}

fn generate_changeset(
    ident: &Ident,
    changeset_options: &ChangesetOptions,
    table_options: &TableOptions,
) -> proc_macro2::TokenStream {
    let (field_idents, field_values, field_declarations, field_functions): (Vec<_>, Vec<_>, Vec<_>, Vec<_>) = table_options
        .fields()
        .filter(|attr| !attr.skip.unwrap_or(false))
        .map(|attr| {
            let mut field_declaration =
                quote! {#[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "::std::option::Option::is_none"))]};

            if attr.is_type_option() {
                field_declaration
                    .extend(quote! {#[cfg_attr(feature = "serde", serde(with = "::serde_with::rust::double_option"))]});
            }

            let field_ident = attr.ident.as_ref().unwrap();
            let field_ty = &attr.ty;

            field_declaration.extend(quote! {
                #field_ident: ::std::option::Option<#field_ty>
            });

            (field_ident, field_ty, field_declaration, changeset_options.setter(field_ident))
        })
        .multiunzip();

    if field_idents.is_empty() {
        quote! {
            #[derive(Debug, Default, Clone)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct #ident;

            impl #ident {
                pub fn is_empty(&self) -> bool { true }
            }
        }
    } else {
        quote! {
            #[derive(Debug, Clone)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct #ident {
                #(
                    #field_declarations
                ),*
            }

            impl Default for #ident {
                fn default() -> Self {
                    Self {
                        #(
                            #field_idents: ::std::option::Option::None
                        ),*
                    }
                }
            }

            impl #ident {
                pub fn is_empty(&self) -> bool {
                    #(
                        self.#field_idents.is_none()
                    )&&*
                }

                #(
                    pub fn #field_functions(mut self, value: impl Into<#field_values>) -> Self {
                        self.#field_idents = Some(value.into());

                        self
                    }
                )*
            }
        }
    }
}
