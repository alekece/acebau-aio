use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::unit::{Unit, Variant};

pub struct UnitImpl<'a> {
    unit: &'a Unit,
    variants: &'a [Variant],
}

impl<'a> UnitImpl<'a> {
    pub fn new(unit: &'a Unit, variants: &'a [Variant]) -> Self {
        Self { unit, variants }
    }
}

impl ToTokens for UnitImpl<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = self.unit.ident();
        let variant_idents = self.variants.iter().map(Variant::ident);
        let factors = self.variants.iter().map(Variant::factor);

        tokens.extend(quote! {
            impl ::acebau_unit::unit::Unit for #ident {
                fn factor(&self) -> ::acebau_unit::Decimal {
                    match self {
                        #(Self::#variant_idents => ::acebau_unit::Decimal::from(#factors),)*
                    }
                }
            }
        });
    }
}
