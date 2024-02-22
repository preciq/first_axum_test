use axum::{body::Body, http::StatusCode, response::{IntoResponse, Response}, routing::{delete, get, post}, Json, Router};
use tokio::net::TcpListener;
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}
//user struct with id, name, and email fields. We also derive the Serialize trait so we can return it as a JSON response.

async fn create_user() -> impl IntoResponse {
    Response::builder()
    .status(StatusCode::CREATED)
    .body(Body::from("User created successfully!"))
    .expect("Unable to create user!")
}
//creates a new user and returns a response with a status code of 201 (CREATED) and a message of "User created successfully!".

async fn list_users() -> Json<Vec<User>> {
    let users = vec![
        User {
            id: 1,
            name: "Elijah".to_string(),
            email: "elijah@example.com".to_string(),
        },
        User {
            id: 2,
            name: "John".to_string(),
            email: "john@doe.com".to_string(),
        },
    ];
    Json(users)
}
//returns a list of users as a JSON response. We create a vector of users and return it as a JSON response.

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, Rust!" }))
        //creates a new route (a new API request point) that is a get request. Creates it at endpoint / and returns "Hello, Rust!" as the response.
        .route("/create_user", post(create_user))
        .route("/list_users", delete(list_users));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    //listens in for requests coming in at port 3000 for the local machine.
    println!("Running on http://localhost:3000");

    axum::serve(listener, app.into_make_service()).await.unwrap();
    //starts the server and listens for requests. It uses the listener and the app we created earlier.
    //kind of like our infinite while loop in previous server iterations.


}
