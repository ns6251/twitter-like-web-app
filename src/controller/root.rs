use axum::{response::Response, routing, Extension, Router};

use crate::{
    controller::tweets,
    database::{self, RepositoryProvider},
    response::into_response,
    services,
};

pub async fn app() -> Router {
    Router::new()
        .route("/", routing::get(get))
        .nest("/tweets", tweets::tweets())
        .layer(database::layer().await)
}

async fn get(Extension(repository_provider): Extension<RepositoryProvider>) -> Response {
    let tweet_repo = repository_provider.tweets();

    into_response(&services::list_tweets(&tweet_repo).await, "")
}
