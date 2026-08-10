use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::unit::{Unit, Variant};

pub struct DisplayImpl<'a> {
    unit: &'a Unit,
    variants: &'a [Variant],
}

impl<'a> DisplayImpl<'a> {
    pub fn new(unit: &'a Unit, variants: &'a [Variant]) -> Self {
        Self { unit, variants }
    }
}

impl ToTokens for DisplayImpl<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = self.unit.ident();
        let arms = self.variants.iter().map(|variant| {
            let variant_ident = variant.ident();
            let symbol = variant.symbol();

            quote! {
                Self::#variant_ident => write!(f, #symbol)
            }
        });

        tokens.extend(quote! {
            impl ::std::fmt::Display for #ident {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        #(#arms,)*
                    }
                }
            }
        });
    }
}
