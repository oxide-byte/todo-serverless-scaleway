use todo_api::{service::todo_service::edit_todo, models::Error};
use todo_api::service::faas_service::setup_tracing;
use sqlx::postgres::{PgPoolOptions, PgPool};
use std::env;
use axum::{routing::put, Router, extract::State, response::IntoResponse, Json};
use tower_http::cors::{Any, CorsLayer};
use http::Method;
use todo_api::models::EditTodo;

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
        .route("/", put(handler))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
                .allow_headers(Any),
        )
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn handler(State(pool): State<PgPool>, Json(todo): Json<EditTodo>) -> impl IntoResponse {
    match edit_todo(pool, todo).await {
        Ok(result) => Json(result).into_response(),
        Err(e) => (http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}