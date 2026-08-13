use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};

use crate::unit::{Unit, Variant};

pub struct ConversionFns<'a> {
    unit: &'a Unit,
    variants: &'a [Variant],
}

impl<'a> ConversionFns<'a> {
    pub fn new(unit: &'a Unit, variants: &'a [Variant]) -> Self {
        Self { unit, variants }
    }
}

impl ToTokens for ConversionFns<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = self.unit.ident();
        let metric_ident = self.unit.metric_ident();
        let fns = self.variants.iter().map(|variant| {
            let variant_ident = variant.ident();
            let from_ident = format_ident!("from_{}", variant.plural());
            let try_from_ident = format_ident!("try_from_{}", variant.plural());
            let to_ident = format_ident!("to_{}", variant.plural());

            quote! {
                pub fn #from_ident<V: ::std::convert::Into<::acebau_unit::Decimal>>(value: V) -> Self {
                    Self::with_unit(value, #ident::#variant_ident)
                }

                pub fn #try_from_ident(value: f64) -> ::std::result::Result<Self, ::acebau_unit::MetricError> {
                    Self::try_with_unit(value, #ident::#variant_ident)
                }

                pub fn #to_ident(self) -> Self {
                    self.convert_to(#ident::#variant_ident)
                }
            }
        });

        tokens.extend(quote! {
            impl #metric_ident {
                #(#fns)*
            }
        });
    }
}
