use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::derive::{UnitDerive, UnitShape, Variant};

pub struct UnitGenerator {
    unit: UnitDerive,
}

impl UnitGenerator {
    pub fn new(unit: UnitDerive) -> Self {
        Self { unit }
    }

    pub fn generate(&self) -> TokenStream {
        match self.unit.shape() {
            UnitShape::Unitless => self.generate_unitless(),
            UnitShape::Enum(variants) => self.generate_enum(variants),
        }
    }

    fn generate_unitless(&self) -> TokenStream {
        let ident = self.unit.ident();
        let metric_ident = self.unit.metric_ident();

        quote! {
            pub type #metric_ident = ::acebau_unit::Metric<::acebau_unit::unit::Unitless<#ident>>;
        }
    }

    fn generate_enum(&self, variants: &[Variant]) -> TokenStream {
        let ident = self.unit.ident();
        let metric_ident = self.unit.metric_ident();
        let variant_idents = variants.iter().map(Variant::ident);
        let factor_consts = variants.iter().map(Variant::factor);
        let display_match_arms = variants.iter().map(|variant| {
            let variant_ident = variant.ident();
            let symbol = variant.symbol();

            quote! {
                Self::#variant_ident => write!(f, #symbol)
            }
        });
        let conversion_fns = variants.iter().map(|variant| {
            let variant_ident = variant.ident();
            let plural_suffix = variant.plural();
            let from_ident = format_ident!("from_{plural_suffix}");
            let to_ident = format_ident!("to_{plural_suffix}");

            quote! {
                pub fn #from_ident(value: f32) -> Self {
                    Self::with_unit(value, #ident::#variant_ident)
                }

                pub fn #to_ident(self) -> Self {
                    self.convert_to(#ident::#variant_ident)
                }
            }
        });

        quote! {
            pub type #metric_ident = ::acebau_unit::Metric<#ident>;

            impl ::acebau_unit::unit::Unit for #ident {
                fn factor(&self) -> f32 {
                    match self {
                        #(
                            Self::#variant_idents => #factor_consts,
                        )*
                    }
                }
            }

            impl ::std::fmt::Display for #ident {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        #(
                            #display_match_arms,
                        )*
                    }
                }
            }

            impl #metric_ident {
                #(
                    #conversion_fns
                )*
            }
        }
    }
}
