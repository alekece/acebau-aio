use actix_web::{get, web::{Json, Path}, Responder, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Material {
    name: String,
}

#[get("/materials/{id}")]
async fn fetch(_id: Path<Uuid>) -> Result<impl Responder> {
    // TODO use backend
    let material = Material {
        name: "wood".to_string(),
    };

    Ok(Json(material))
}

#[get("/materials")]
async fn fetch_all() -> Result<impl Responder> {
    // TODO use backend
    let materials = vec![
        Material {
            name: "wood".to_string(),
        },
        Material {
            name: "metal".to_string(),
        },
        Material {
            name: "plastic".to_string(),
        },
    ];

    Ok(Json(materials))
}
