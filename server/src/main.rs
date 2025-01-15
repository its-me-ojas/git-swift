use std::env;

use axum::Router;
use server::{errors::Result, routes};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()>{
    dotenvy::dotenv().unwrap();

    let routes_all = Router::new().merge(routes::commit_message::route());

    let port = env::var("PORT").unwrap_or(String::from("5000"));

    let listener = TcpListener::bind(&format!("127.0.0.1:{}", port)).await.unwrap();
    println!("The server is listening on {:?}", listener.local_addr().unwrap());

    axum::serve(listener, routes_all.into_make_service()).await.unwrap();

    Ok(())
}