use axum::{Router, routing::{get}};
use tokio::net::TcpListener;

#[tokio::main]

async fn main(){
    let router01: Router = Router::new()
        .route("/vehicle", get(get_vehicle).post(post_vehicle));
    
    let address: &str = "0.0.0.0:6570";
    let listener: TcpListener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router01).await.unwrap();

}

async fn get_vehicle(){
    
}


async fn post_vehicle(){
    
}