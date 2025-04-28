mod api_handler;

use std::{net::SocketAddr,  sync::{Arc, Mutex}};

use api_handler::UserList;
use axum::{
    response::Html, routing::{delete, get, post, put}, Router
};

#[tokio::main]
async fn main() {
    let users: UserList =  Arc::new(Mutex::new(vec![]));

    let app = Router::new()
        .route("/test-api", get(api_handler::hello))

        .route("/hello-world", get( || async {
            Html("Hello World!")
        }))

        .route("/user", post(api_handler::insert_user_data))
        .route("/user", get(api_handler::get_users))
        .route("/user/{id}", get(api_handler::get_user))
        .route("/user/{id}", put(api_handler::update_user))
        .route("/user/{id}", delete(api_handler::delete_user))
        .with_state(users.clone());

    let addr = SocketAddr::from(([127,0,0,1], 3000));
    println!("->> Listening on {addr}\n");

    // axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
    // .serve(app.into_make_service())
    // .await.unwrap();


    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
