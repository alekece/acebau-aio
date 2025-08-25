use actix_web::{
    HttpRequest, HttpResponse, Responder,
    body::EitherBody,
    error::JsonPayloadError,
    mime,
    web::{Json, Query},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct FieldMask {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope<T>(pub T);

impl<T> Envelope<T> {}

impl<T: Serialize> Responder for Envelope<T> {
    type Body = EitherBody<String>;

    fn respond_to(self, request: &HttpRequest) -> HttpResponse<Self::Body> {
        match (
            serde_json::to_value(&self.0),
            Query::<FieldMask>::from_query(request.query_string()),
        ) {
            (Ok(mut value), Ok(field_mask)) => {
                if !field_mask.is_empty() {
                    field_mask.project(&mut value);
                }
                /*
                                if let Value::Array(values) = value {
                                    value = json!({"items": values});
                                }
                */
                match HttpResponse::Ok()
                    .content_type(mime::APPLICATION_JSON)
                    .message_body(serde_json::to_string(&value).unwrap())
                {
                    Ok(response) => response.map_into_left_body(),
                    Err(error) => HttpResponse::from_error(error).map_into_right_body(),
                }
            }
            (Err(error), _) => HttpResponse::from_error(JsonPayloadError::Serialize(error)).map_into_right_body(),
            (_, Err(error)) => HttpResponse::from_error(error).map_into_right_body(),
        }
    }
}
