//! Proc-macro implementation for `acebau_unit` derives.

mod generate;
mod unit;

use darling::FromDeriveInput;
use quote::ToTokens;
use syn::{DeriveInput, parse_macro_input};

use self::unit::Unit;

#[proc_macro_derive(Unit, attributes(unit))]
pub fn derive_unit(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    Unit::from_derive_input(&input)
        .map(ToTokens::into_token_stream)
        .unwrap_or_else(|error| error.write_errors())
        .into()
}
