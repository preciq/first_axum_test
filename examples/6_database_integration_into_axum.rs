use std::time::Instant;

use axum::{response::IntoResponse, routing::get, Extension, Json, Router};
use serde_json::json;
use sqlx::{MySqlPool, Row};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let database_url = "mysql://root:Rust_12345@0.tcp.ngrok.io:11872/mysql";
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Could not connect to the database");

    let app = Router::new()
        .route("/users", get(get_users))
        .layer(Extension(pool));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    //listens in for requests coming in at port 3000 for the local machine.
    println!("Running on http://localhost:3000");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
    //starts the server and listens for requests. It uses the listener and the app we created earlier.
    //kind of like our infinite while loop in previous server iterations.
}

// Define the get_users function as before
async fn get_users(Extension(pool): Extension<MySqlPool>) -> impl IntoResponse {
    let start = Instant::now();

    let rows = match sqlx::query("SELECT id, name, email FROM users LIMIT 2")
        .fetch_all(&pool)
        .await
    {
        Ok(rows) => rows,
        Err(_) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            )
                .into_response()
        }
    };

    let users: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|row| {
            json!({
                "id": row.try_get::<i32, _>("id").unwrap_or_default(),
                "name": row.try_get::<String, _>("name").unwrap_or_default(),
                "email": row.try_get::<String, _>("email").unwrap_or_default(),
            })
        })
        .collect();

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

    (axum::http::StatusCode::OK, Json(users)).into_response()
}
