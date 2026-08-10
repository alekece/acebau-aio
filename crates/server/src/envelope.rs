use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use serde_with::{StringWithSeparator, formats::CommaSeparator, serde_as};

#[serde_as]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct FieldMask {
    #[serde_as(as = "Option<StringWithSeparator<CommaSeparator, String>>")]
    fields: Option<Vec<String>>,
}

impl FieldMask {
    fn is_empty(&self) -> bool {
        self.fields.as_ref().map(|fields| fields.is_empty()).unwrap_or(true)
    }

    fn project(&self, value: &mut Value) {
        match value {
            Value::Object(map) => {
                if let Some(fields) = &self.fields {
                    map.retain(|k, _| fields.contains(k));
                }
            }
            Value::Array(array) => {
                for value in array.iter_mut() {
                    self.project(value);
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
pub struct Envelope<T> {
    data: T,
    status: StatusCode,
}

impl<T> Envelope<T> {
    pub fn ok(data: T) -> Self {
        Self {
            data,
            status: StatusCode::OK,
        }
    }

    pub fn created(data: T) -> Self {
        Self {
            data,
            status: StatusCode::CREATED,
        }
    }
}

impl<T: Serialize> Envelope<T> {
    pub(crate) fn into_response(self, field_mask: &FieldMask) -> Response {
        let Ok(mut value) = serde_json::to_value(&self.data) else {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        };

        if !field_mask.is_empty() {
            field_mask.project(&mut value);
        }

        if let Value::Array(values) = value {
            value = json!({ "items": values });
        }

        (self.status, axum::Json(value)).into_response()
    }
}
