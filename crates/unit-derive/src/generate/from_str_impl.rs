use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::unit::{Unit, Variant};

pub struct FromStrImpl<'a> {
    unit: &'a Unit,
    variants: &'a [Variant],
}

impl<'a> FromStrImpl<'a> {
    pub fn new(unit: &'a Unit, variants: &'a [Variant]) -> Self {
        Self { unit, variants }
    }
}

impl ToTokens for FromStrImpl<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = self.unit.ident();
        let arms = self.variants.iter().map(|variant| {
            let variant_ident = variant.ident();
            let symbol = variant.symbol();

            quote! {
                #symbol => ::std::result::Result::Ok(Self::#variant_ident)
            }
        });

        tokens.extend(quote! {
            impl ::std::str::FromStr for #ident {
                type Err = ::acebau_unit::unit::UnitParseError;

                fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
                    match value {
                        #(#arms,)*
                        _ => ::std::result::Result::Err(
                            ::acebau_unit::unit::UnitParseError::new(value)
                        ),
                    }
                }
            }
        });
    }
}
