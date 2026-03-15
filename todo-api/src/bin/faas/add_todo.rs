use todo_api::{service::todo_service::add_todo, models::Error};
use todo_api::service::faas_service::setup_tracing;
use sqlx::postgres::{PgPoolOptions, PgPool};
use std::env;
use axum::{routing::post, Router, extract::State, response::IntoResponse, Json};
use tower_http::cors::{Any, CorsLayer};
use todo_api::models::NewTodo;

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
        .route("/", post(handler))
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

async fn handler(State(pool): State<PgPool>, Json(todo): Json<NewTodo>) -> impl IntoResponse {
    match add_todo(pool, todo).await {
        Ok(result) => Json(result).into_response(),
        Err(e) => (http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}