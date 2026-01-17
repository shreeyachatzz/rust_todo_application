use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post, patch, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;

// --- Data Models ---
#[derive(Serialize, Deserialize, FromRow, ToSchema, Clone)]
struct Todo {
    id: Uuid,
    title: String,
    completed: bool,
}

#[derive(Deserialize, ToSchema)]
struct CreateTodo {
    title: String,
}

#[derive(Deserialize, ToSchema)]
struct UpdateTodo {
    completed: bool,
}

// --- API Documentation Struct ---
#[derive(OpenApi)]
#[openapi(
    paths(get_todos, create_todo, update_todo, delete_todo),
    components(schemas(Todo, CreateTodo, UpdateTodo))
)]
struct ApiDoc;

// --- Main Application ---
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Connect to the Dockerized DB
    let pool = PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Failed to connect to DB");

    // Allow Frontend to talk to Backend
    let cors = CorsLayer::permissive();

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/todos", get(get_todos).post(create_todo))
        .route("/todos/{id}", patch(update_todo).delete(delete_todo))
        .layer(cors)
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    println!("Swagger docs at http://localhost:3000/swagger-ui");
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// --- Handlers ---

#[utoipa::path(
    get, path = "/todos",
    responses((status = 200, description = "List all todos", body = [Todo]))
)]
async fn get_todos(State(pool): State<PgPool>) -> Json<Vec<Todo>> {
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos")
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(todos)
}

#[utoipa::path(
    post, path = "/todos", request_body = CreateTodo,
    responses((status = 201, description = "Todo created", body = Todo))
)]
async fn create_todo(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTodo>,
) -> (StatusCode, Json<Todo>) {
    let new_id = Uuid::new_v4();
    let todo = sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (id, title) VALUES ($1, $2) RETURNING *"
    )
    .bind(new_id)
    .bind(payload.title)
    .fetch_one(&pool)
    .await
    .unwrap();

    (StatusCode::CREATED, Json(todo))
}

#[utoipa::path(
    patch, path = "/todos/{id}", request_body = UpdateTodo,
    responses((status = 200, description = "Todo updated"))
)]
async fn update_todo(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTodo>,
) -> StatusCode {
    sqlx::query("UPDATE todos SET completed = $1 WHERE id = $2")
        .bind(payload.completed)
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();
    StatusCode::OK
}

#[utoipa::path(
    delete, path = "/todos/{id}",
    responses((status = 204, description = "Todo deleted"))
)]
async fn delete_todo(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> StatusCode {
    sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();
    StatusCode::NO_CONTENT
}