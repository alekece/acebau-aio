use derive_more::From;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::unit::{Unit, UnitShape};

#[derive(From)]
pub struct MetricAlias<'a>(&'a Unit);

impl ToTokens for MetricAlias<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = self.0.ident();
        let metric_ident = self.0.metric_ident();

        let alias = match self.0.shape() {
            UnitShape::Unitless => {
                quote! {
                    ::acebau_unit::Metric<::acebau_unit::unit::Unitless<#ident>>
                }
            }
            UnitShape::Enum(_) => {
                quote! {
                    ::acebau_unit::Metric<#ident>
                }
            }
        };

        tokens.extend(quote! {
            pub type #metric_ident = #alias;
        });
    }
}
