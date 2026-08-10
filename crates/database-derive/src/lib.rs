//! Proc-macro implementation for database table derives.

mod generate;
mod table;

use darling::FromDeriveInput;
use quote::ToTokens;
use syn::{DeriveInput, parse_macro_input};

use self::table::Table;

#[proc_macro_derive(Table, attributes(table, changeset))]
pub fn derive_table(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    Table::from_derive_input(&input)
        .map(ToTokens::into_token_stream)
        .unwrap_or_else(|error| error.write_errors())
        .into()
}
