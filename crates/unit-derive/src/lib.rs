use darling::{
    FromDeriveInput, FromVariant,
    ast::{Data, Fields},
    util::Ignored,
};
use itertools::Itertools;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::{format_ident, quote};
use syn::{DeriveInput, Ident, Visibility, parse_macro_input};

#[derive(FromDeriveInput)]
#[darling(attributes(unit), supports(enum_any, struct_any))]
struct UnitOptions {
    ident: Ident,
    vis: Visibility,
    data: Data<VariantOptions, Ignored>,
}

impl UnitOptions {
    fn metric_ident(&self) -> Ident {
        self.ident
            .to_string()
            .strip_suffix("Unit")
            .map(|name| Ident::new(name, self.ident.span()))
            .unwrap_or_else(|| format_ident!("{}Metric", self.ident))
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
#[proc_macro_derive(Unit, attributes(unit))]
pub fn derive_unit(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let unit_options = match UnitOptions::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let ident = &unit_options.ident;
    let vis = &unit_options.vis;
    let metric_ident = unit_options.metric_ident();

    let output = match &unit_options.data {
        Data::Struct(Fields { fields, .. }) => {
            if !fields.is_empty() {
                return darling::Error::custom("Unit can only be derived for enums or empty structs")
                    .with_span(ident)
                    .write_errors()
                    .into();
            }

            quote! {
                #vis type #metric_ident = ::acebau_unit::Metric<::acebau_unit::unit::Unitless<#ident>>;
            }
        }
        Data::Enum(variants) => {
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
                #vis type #metric_ident = ::acebau_unit::Metric<#ident>;

                impl ::acebau_unit::unit::Unit for #ident {
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

                impl #metric_ident {
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
        }
    };

    output.into()
}
