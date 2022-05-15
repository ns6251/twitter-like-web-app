mod controller;
mod database;
mod entities;
mod repos_impl;
mod repositories;
mod response;
mod services;
mod views;

pub use controller::app;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "rustwi=debug");
    }

    tracing_subscriber::fmt::init();

    let host = std::env::var("HOST").unwrap();
    let port = std::env::var("PORT").unwrap();
    let addr = format!("{}:{}", host, port).parse().unwrap();

    tracing::debug!("listening on {}", addr);
    let app = crate::app().await;

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
