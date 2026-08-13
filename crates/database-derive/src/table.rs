use darling::{
    Error, FromDeriveInput, FromField, FromMeta,
    ast::{Data, Fields},
    util::Ignored,
};
use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Ident, Path, Type};

use crate::generate::{ChangesetStruct, InputStruct, MutationStruct, OutputImpl, QueryStruct, RepositoryImpl};

#[derive(FromField)]
#[darling(attributes(table), and_then = Self::finalize)]
pub struct Field {
    ident: Option<Ident>,
    ty: Type,
    #[darling(default)]
    rename: Option<String>,
    #[darling(default)]
    skip: bool,
    #[darling(rename = "relationship", default)]
    relation: Option<Relationship>,
    #[darling(skip)]
    name: Option<String>,
}

impl Field {
    pub fn ident(&self) -> &Ident {
        self.ident.as_ref().unwrap()
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }

    pub fn name(&self) -> &str {
        self.name.as_deref().unwrap()
    }

    pub fn skip(&self) -> bool {
        self.skip
    }

    pub fn relationship(&self) -> Option<&Relationship> {
        self.relation.as_ref()
    }

    pub fn is_optional(&self) -> bool {
        matches!(
            &self.ty,
            Type::Path(path) if path.path.segments.last().is_some_and(|segment| segment.ident == "Option")
        )
    }

    fn finalize(mut self) -> Result<Self, Error> {
        self.name = Some(
            self.rename
                .take()
                .unwrap_or_else(|| self.ident().to_string().to_snake_case()),
        );
        Ok(self)
    }
}

#[derive(Debug, FromMeta)]
pub struct Relationship {
    name: Ident,
    target: Path,
}

impl Relationship {
    pub fn name(&self) -> &Ident {
        &self.name
    }

    pub fn target(&self) -> &Path {
        &self.target
    }
}

#[derive(FromDeriveInput)]
#[darling(attributes(table, changeset), supports(struct_named), and_then = Self::finalize)]
pub struct Table {
    ident: Ident,
    name: String,
    #[darling(default)]
    plural: Option<String>,
    data: Data<Ignored, Field>,
    #[darling(default)]
    setter: Option<SetterOptions>,
    #[darling(rename = "resolver", multiple)]
    resolvers: Vec<Resolver>,
    #[darling(skip)]
    changeset_ident: Option<Ident>,
}

impl Table {
    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn plural_name(&self) -> String {
        self.plural.clone().unwrap_or_else(|| format!("{}s", self.name))
    }

    pub fn changeset_ident(&self) -> &Ident {
        self.changeset_ident.as_ref().unwrap()
    }

    pub fn input_ident(&self) -> Ident {
        format_ident!("{}Input", self.ident)
    }

    pub fn query_ident(&self) -> Ident {
        format_ident!("{}Query", self.ident)
    }

    pub fn mutation_ident(&self) -> Ident {
        format_ident!("{}Mutation", self.ident)
    }

    pub fn setter(&self, ident: &Ident) -> Ident {
        self.setter
            .as_ref()
            .and_then(|setter| setter.prefix.as_deref())
            .map_or_else(|| ident.clone(), |prefix| format_ident!("{prefix}_{ident}"))
    }

    pub fn fields(&self) -> impl Iterator<Item = &Field> {
        let Data::Struct(Fields { fields, .. }) = &self.data else {
            unreachable!("Table can only be derived for structs");
        };

        fields.iter()
    }

    pub fn resolvers(&self) -> impl Iterator<Item = &Resolver> {
        self.resolvers.iter()
    }

    fn finalize(mut self) -> Result<Self, Error> {
        self.changeset_ident = Some(format_ident!("{}Changeset", self.ident));

        Ok(self)
    }
}

#[derive(FromMeta)]
pub struct Resolver {
    name: Ident,
    method: Ident,
    ty: Type,
}

impl Resolver {
    pub fn name(&self) -> &Ident {
        &self.name
    }

    pub fn method(&self) -> &Ident {
        &self.method
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }
}

#[derive(FromMeta)]
struct SetterOptions {
    #[darling(default)]
    prefix: Option<String>,
}

impl ToTokens for Table {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let changeset_struct = ChangesetStruct::from(self);
        let input_struct = InputStruct::from(self);
        let mutation_struct = MutationStruct::from(self);
        let output_impl = OutputImpl::from(self);
        let query_struct = QueryStruct::from(self);
        let repository_impl = RepositoryImpl::from(self);

        tokens.extend(quote! {
            #changeset_struct
            #input_struct
            #mutation_struct
            #output_impl
            #query_struct
            #repository_impl
        });
    }
}

#[cfg(test)]
mod tests {
    use darling::FromDeriveInput;
    use quote::quote;
    use syn::{DeriveInput, parse2};

    use super::Table;

    #[test]
    fn accepts_multiple_resolvers() {
        let input: DeriveInput = parse2(quote! {
            #[table(
                name = "example",
                resolver(name = first, method = resolve_first, ty = "String"),
                resolver(name = second, method = resolve_second, ty = "i32")
            )]
            struct Example {
                value: String,
            }
        })
        .unwrap();

        let table = Table::from_derive_input(&input).unwrap();
        let names = table
            .resolvers()
            .map(|resolver| resolver.name().to_string())
            .collect::<Vec<_>>();

        assert_eq!(names, ["first", "second"]);
    }
}
