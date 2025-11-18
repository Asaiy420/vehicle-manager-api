use axum::{ Router, routing::get};
use tokio::net::TcpListener;

mod vehicle;

#[tokio::main]

async fn main(){
    let router01: Router = Router::new()
        .route("/vehicle", get(vehicle::get_vehicle).post(vehicle::post_vehicle));
    
    let address: &str = "0.0.0.0:6570";
    let listener: TcpListener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router01).await.unwrap();

}
