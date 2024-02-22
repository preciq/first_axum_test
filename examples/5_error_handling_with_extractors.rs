use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::delete, Json, Router};
use tokio::net::TcpListener;

use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
}
//Will create an instance of User to represent a deleted user. This will be serialized and sent as part of the response.

async fn delete_user(Path(user_id): Path<u64>) -> Result<Json<User>, impl IntoResponse> {
        match perform_delete_user(user_id).await {
        Ok(_) => Ok(Json(User {
            id: user_id,
            name: "Deleted User".into(),
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete user: {}", e),
        )),
    }
}
//This function will delete a user by ID and return a response. It will return a 500 status code if the user cannot be deleted.

// Hypothetical async function to delete a user by ID
async fn perform_delete_user(user_id: u64) -> Result<(), String> {
    // Simulate an error for demonstration
    if user_id == 1 {
        Err("User cannot be deleted.".to_string())
    } else {
        // Logic to delete a user...
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/delete-user/:user_id", delete(delete_user));
    //creates a new route (a new API request point) that is a delete request. Creates it at endpoint /delete-user/:user_id and returns a response with the user that was deleted.
    
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    //listens in for requests coming in at port 3000 for the local machine.
    println!("Running on http://localhost:3000");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
    //starts the server and listens for requests. It uses the listener and the app we created earlier.
    //kind of like our infinite while loop in previous server iterations.
}