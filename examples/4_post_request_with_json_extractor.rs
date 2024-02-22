use axum::{routing::post, Json, Router};
use serde::Deserialize;
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct Item {
    title: String,
}
//this will be the template with which the JSON data will be extracted from the request body.

async fn add_item(Json(item): Json<Item>) -> String {
    format!("Added item with title: {}", item.title)
}
//

#[tokio::main]
async fn main() {
    let app = Router::new().route("/add-item", post(add_item));
    //creates a new route (a new API request point) that is a post request. Creates it at endpoint /add-item and returns a response with the title of the item that was added.

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    //listens in for requests coming in at port 3000 for the local machine.
    println!("Running on http://localhost:3000");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
    //starts the server and listens for requests. It uses the listener and the app we created earlier.
    //kind of like our infinite while loop in previous server iterations.
}
