use axum::{Json, Router, routing::get, debug_handler};
use tokio::net::TcpListener;
use serde::Serialize;

#[tokio::main]

async fn main(){
    let router01: Router = Router::new()
        .route("/vehicle", get(get_vehicle).post(post_vehicle));
    
    let address: &str = "0.0.0.0:6570";
    let listener: TcpListener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router01).await.unwrap();

}
#[derive(Serialize)]
struct Vehicle{
    manufacturer: String,
    model: String,
    year: u32,
    id: String,
}

#[debug_handler]
async fn get_vehicle() -> Json<Vehicle>{
    Json::from(
    Vehicle{
        manufacturer: String::from("Triumph"),
        model: String::from("Triumph Scrambler 400x"),
        year: 2024,
        id: uuid::Uuid::new_v4().to_string(),
    })
}


async fn post_vehicle(){
    
}