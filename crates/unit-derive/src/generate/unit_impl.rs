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
        let name_const = self.unit.metric_ident().to_string();
        let variant_idents = self.variants.iter().map(Variant::ident);
        let factors = self.variants.iter().map(|variant| {
            let (lo, mid, hi, scale) = variant.factor().parts();
            quote! { ::acebau_unit::Decimal::from_parts(#lo, #mid, #hi, false, #scale) }
        });

        tokens.extend(quote! {
            impl ::acebau_unit::unit::Unit for #ident {
                const NAME: &'static str = #name_const;

                fn factor(&self) -> ::acebau_unit::Decimal {
                    match self {
                        #(Self::#variant_idents => #factors,)*
                    }
                }
            }
        });
    }
}
