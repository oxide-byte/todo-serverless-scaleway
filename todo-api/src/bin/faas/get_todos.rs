use todo_api::{service::todo_service::get_todos, models::Error};
use todo_api::service::faas_service::setup_tracing;
use sqlx::postgres::{PgPoolOptions, PgPool};
use std::env;
use axum::{routing::get, Router, extract::State, response::IntoResponse, Json};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:password@localhost:5432/todo-db".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(&database_url)
        .await?;

    tracing::info!("Connected to database");

    let app = Router::new()
        .route("/", get(handler))
        .with_state(pool)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn handler(State(pool): State<PgPool>) -> impl IntoResponse {
    match get_todos(pool).await {
        Ok(todos) => Json(todos).into_response(),
        Err(e) => (http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}