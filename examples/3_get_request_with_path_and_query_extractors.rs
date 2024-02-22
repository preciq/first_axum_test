use axum::{
    extract::{Path, Query},
    routing::get,
    Router,
};
use serde::Deserialize;
use tokio::net::TcpListener;

// A struct for query parameters
#[derive(Deserialize)]
struct Page {
    number: u32,
}

/*
This method extracts parameters passed in as part of the request
It extracts both Path paramters and Query parameters
I.e. in the get request: /item/hello?number=300
 - hello is the Path parameter
 - 300 is the Query parameter
*/
async fn show_item(Path(id): Path<String>, Query(page): Query<Page>) -> String {
    format!("Item {} on page {}", id, page.number)
}


#[tokio::main]
async fn main() {
    let app = Router::new().route("/item/:id", get(show_item));
    //:id in this case means a parameter. It is a placeholder for the actual value that will be passed in the request.

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    //listens in for requests coming in at port 3000 for the local machine.
    println!("Running on http://localhost:3000");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
    //starts the server and listens for requests. It uses the listener and the app we created earlier.
    //kind of like our infinite while loop in previous server iterations.
}
