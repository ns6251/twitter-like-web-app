use axum::{
    extract::{Form, Path},
    response::{IntoResponse, Redirect},
    routing, Extension, Router,
};
use serde::Deserialize;

use crate::{database::RepositoryProvider, services};

pub fn tweets() -> Router {
    Router::new()
        .route("/new", routing::post(post))
        .route("/:id/delete", routing::post(delete))
}

async fn post(
    form: Form<TweetForm>,
    Extension(repo): Extension<RepositoryProvider>,
) -> impl IntoResponse {
    let tweets = repo.tweets();
    services::create_tweet(&tweets, form.message.as_str()).await;
    Redirect::to("/")
}

async fn delete(
    Path(id): Path<i32>,
    Extension(repo): Extension<RepositoryProvider>,
) -> impl IntoResponse {
    let tweets = repo.tweets();
    services::delete_tweet(&tweets, id).await;
    Redirect::to("/")
}

#[derive(Deserialize)]
struct TweetForm {
    message: String,
}
