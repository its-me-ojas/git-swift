use std::env;
use std::time::Instant;

use axum::{
    http::Request,
    middleware::{self, Next},
    response::Response,
    Router,
};
use chrono;
use server::{errors::Result, routes};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().unwrap();

    let routes_all = Router::new()
        .merge(routes::commit_message::route())
        .layer(middleware::from_fn(log_request)); // Add the middleware here!
    let port = env::var("PORT").unwrap_or(String::from("5000"));

    let listener = TcpListener::bind(&format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    println!(
        "The server is listening on {:?}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn log_request(req: Request<axum::body::Body>, next: Next) -> Result<Response> {
    let start = Instant::now();
    let path = req.uri().path().to_owned();
    let method = req.method().clone();

    let response = next.run(req).await;

    let status = response.status();
    let elapsed = start.elapsed();

    println!(
        "{} {} {} {} {}ms",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        method,
        path,
        status,
        elapsed.as_millis()
    );

    Ok(response)
}