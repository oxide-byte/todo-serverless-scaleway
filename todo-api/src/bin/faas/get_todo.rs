use todo_api::{service::todo_service::get_todo, models::Error};
use todo_api::service::faas_service::setup_tracing;
use sqlx::postgres::{PgPoolOptions, PgPool};
use std::env;
use axum::{routing::get, Router, extract::{State, Query}, response::IntoResponse, Json};
use serde::Deserialize;

#[derive(Deserialize)]
struct Params {
    id: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://todo:password@localhost:5432/todo".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let app = Router::new()
        .route("/", get(handler))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn handler(State(pool): State<PgPool>, Query(params): Query<Params>) -> impl IntoResponse {
    match get_todo(pool, params.id).await {
        Ok(todo) => Json(todo).into_response(),
        Err(e) => (http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}