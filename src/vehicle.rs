
use axum::{Json, debug_handler};
use serde::Serialize;

#[derive(Serialize)]
pub struct Vehicle{
    
    manufacturer: String,
    model: String,
    year: u32,
    id: String,
}

#[debug_handler]
pub async fn get_vehicle() -> Json<Vehicle>{
    Json::from(
    Vehicle{
        manufacturer: String::from("Triumph"),
        model: String::from("Triumph Scrambler 400x"),
        year: 2024,
        id: uuid::Uuid::new_v4().to_string(),
    })
}

pub async fn post_vehicle(){
    
}