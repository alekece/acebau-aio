use async_graphql::{
    InputType,
    indexmap::IndexMap,
    registry::{Deprecation, MetaInputValue, MetaType, MetaTypeId, Registry},
};

pub(crate) fn create_meta_input_value<T: InputType>(registry: &mut Registry, name: &str) -> (String, MetaInputValue) {
    (
        name.to_owned(),
        MetaInputValue {
            name: name.to_owned(),
            description: None,
            ty: T::create_type_info(registry),
            deprecation: Deprecation::NoDeprecated,
            default_value: None,
            visible: None,
            inaccessible: false,
            tags: Vec::new(),
            is_secret: false,
            directive_invocations: Vec::new(),
        },
    )
}

pub(crate) fn create_input_type<T: InputType>(
    registry: &mut Registry,
    input_fields: IndexMap<String, MetaInputValue>,
) -> String {
    registry.create_input_type::<T, _>(MetaTypeId::InputObject, move |_| MetaType::InputObject {
        name: T::type_name().into_owned(),
        description: None,
        input_fields: input_fields.clone(),
        visible: None,
        inaccessible: false,
        tags: Vec::new(),
        rust_typename: Some(std::any::type_name::<T>()),
        oneof: false,
        directive_invocations: Vec::new(),
    })
}
