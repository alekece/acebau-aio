use darling::{FromDeriveInput, FromVariant, ast::Data};
use itertools::Itertools;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::{format_ident, quote};
use syn::{DeriveInput, Ident, parse_macro_input};

#[derive(FromDeriveInput)]
#[darling(attributes(unit), supports(enum_any))]
struct UnitOptions {
    ident: Ident,
    data: Data<VariantOptions, ()>,
}

impl UnitOptions {
    fn variants(&self) -> &[VariantOptions] {
        let Data::Enum(variants) = &self.data else {
            unreachable!("Unit can only be used with enums");
        };

        &variants[..]
    }
}

#[derive(FromVariant)]
#[darling(attributes(unit))]
struct VariantOptions {
    ident: Ident,
    symbol: String,
    factor: f32,
}

#[proc_macro_error]
#[proc_macro_derive(Table, attributes(unit))]
pub fn derive_unit(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let unit_options = match UnitOptions::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let ident = &unit_options.ident;
    let variants = unit_options.variants();

    let output = if variants.is_empty() {
        quote! {
            impl ::acebau_unit::Unit for #ident {
                fn factor(&self) -> f32 {
                    1.
                }
            }

            impl ::std::fmt::Display for #ident {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, "")
                }
            }
        }
    } else {
        let (variant_idents, factors, symbols, from_idents, to_idents): (Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>) =
            variants
                .iter()
                .map(|variant| {
                    let plural = format_ident!("{}s", variant.ident.to_string().to_lowercase());

                    (
                        &variant.ident,
                        variant.factor,
                        &variant.symbol,
                        format_ident!("from_{}", plural),
                        format_ident!("to_{}", plural),
                    )
                })
                .multiunzip();

        quote! {
            impl ::acebau_unit::Unit for #ident {
                fn factor(&self) -> f32 {
                    match self {
                        #(
                            Self::#variant_idents => #factors,
                        )*
                    }
                }
            }

            impl ::std::fmt::Display for #ident {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        #(
                            Self::#variant_idents => write!(f, "{}", #symbols),
                        )*
                    }
                }
            }

            impl ::acebau_unit::Metric<#ident> {
                #(
                    pub fn #from_idents(value: f32) -> Self {
                        Self::with_unit(value, #ident::#variant_idents)
                    }

                    pub fn #to_idents(self) -> Self {
                        self.convert_to(#ident::#variant_idents)
                    }
                )*
            }
        }
    };

    output.into()
}
