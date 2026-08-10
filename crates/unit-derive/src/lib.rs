//! Proc-macro implementation for `acebau_unit` derives.

mod derive;
mod generator;

use darling::FromDeriveInput;
use syn::{DeriveInput, parse_macro_input};

use self::{derive::UnitDerive, generator::UnitGenerator};

#[proc_macro_derive(Unit, attributes(unit))]
pub fn derive_unit(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    UnitDerive::from_derive_input(&input)
        .map(|unit| UnitGenerator::new(unit).generate())
        .unwrap_or_else(|error| error.write_errors())
        .into()
}
