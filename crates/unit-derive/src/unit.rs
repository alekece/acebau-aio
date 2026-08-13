use darling::{
    Error, FromDeriveInput, FromVariant,
    ast::{Data, Fields},
    util::Ignored,
};
use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident};
use syn::Ident;

use crate::generate::{ConversionFns, DisplayImpl, FromStrImpl, MetricAlias, UnitImpl};

pub enum UnitShape<'a> {
    Unitless,
    Enum(&'a [Variant]),
}

#[derive(FromVariant)]
#[darling(attributes(unit), and_then = Self::finalize)]
pub struct Variant {
    ident: Ident,
    symbol: String,
    factor: u32,
    #[darling(default)]
    plural: Option<String>,
}

impl Variant {
    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn factor(&self) -> u32 {
        self.factor
    }

    pub fn plural(&self) -> &str {
        self.plural.as_deref().unwrap()
    }

    fn finalize(mut self) -> Result<Self, Error> {
        self.plural
            .get_or_insert_with(|| format!("{}s", self.ident.to_string().to_snake_case()));

        Ok(self)
    }
}

#[derive(FromDeriveInput)]
#[darling(attributes(unit), supports(enum_any, struct_any), and_then = Self::finalize)]
pub struct Unit {
    ident: Ident,
    data: Data<Variant, Ignored>,
    #[darling(skip)]
    metric_ident: Option<Ident>,
}

impl Unit {
    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn metric_ident(&self) -> &Ident {
        self.metric_ident.as_ref().unwrap()
    }

    pub fn shape(&self) -> UnitShape<'_> {
        match &self.data {
            Data::Enum(variants) => UnitShape::Enum(variants),
            Data::Struct(_) => UnitShape::Unitless,
        }
    }

    fn finalize(mut self) -> Result<Self, Error> {
        if let Data::Struct(Fields { fields, .. }) = &self.data
            && !fields.is_empty()
        {
            return Err(Error::custom("Unit can only be derived for enums or empty structs").with_span(&self.ident));
        }

        self.metric_ident = Some(
            self.ident
                .to_string()
                .strip_suffix("Unit")
                .map(|name| Ident::new(name, self.ident.span()))
                .unwrap_or_else(|| format_ident!("{}Metric", self.ident)),
        );

        Ok(self)
    }
}

impl ToTokens for Unit {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        MetricAlias::from(self).to_tokens(tokens);

        if let UnitShape::Enum(variants) = self.shape() {
            UnitImpl::new(self, variants).to_tokens(tokens);
            DisplayImpl::new(self, variants).to_tokens(tokens);
            FromStrImpl::new(self, variants).to_tokens(tokens);
            ConversionFns::new(self, variants).to_tokens(tokens);
        }
    }
}
