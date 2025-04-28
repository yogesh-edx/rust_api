
use std::sync::{Arc, Mutex};

use axum::{extract::{Json, Path, State}, http::StatusCode, response::IntoResponse };
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

pub type UserList = Arc<Mutex<Vec<User>>>;
// let users: UserList = Arc::new(Mutex::new(vec![]));

pub async fn get_users(State(users): State<UserList>) -> impl IntoResponse {
    let data = users.lock().unwrap();
    Json(data.clone())
}

//Get user by ID
pub async fn get_user(Path(id): Path<u32>, State(users): State<UserList>) -> impl IntoResponse {
    let users = users.lock().unwrap();
    let users_data = users.iter().find(|u| u.id == id);
    println!(" ---> {:?}", users_data);
    // if let Some(user) = users.iter().find(|u| u.id == id) {
    if Some(users_data).is_some() {
        Json(users_data.clone()).into_response()
    } else {
        (StatusCode::NOT_FOUND, "User not found").into_response()
    }
}

pub async fn insert_user_data(State(users): State<UserList>, Json(new_user): Json<User>)-> impl IntoResponse {
    // let user_vect
    let mut users = users.lock().unwrap();
    users.push(new_user.clone());
    (StatusCode::CREATED, Json(new_user))
}


// //Update user by ID
// pub async fn update_user(Path(id): Path<u32>, State(users): State<UserList>) -> impl IntoResponse {
//     let users = users.lock().unwrap();
//     let users_data = users.iter().find(|u| u.id == id);
//     println!(" ---> {:?}", users_data);
//     // if let Some(user) = users.iter().find(|u| u.id == id) {
//     if Some(users_data).is_some() {
//         Json(users_data.clone()).into_response()
//     } else {
//         (StatusCode::NOT_FOUND, "User not found").into_response()
//     }
// }

pub async fn update_user(Path(id): Path<u32>, State(users): State<UserList>, Json(payload): Json<User>) -> impl IntoResponse {
    let mut users = users.lock().unwrap();
    // let users_data = users.iter().find(|u| u.id == id);
    if let Some(user) = users.iter_mut().find(|u| u.id == id) {
        user.name = payload.name;
        user.email = payload.email;
        return Json(user.clone()).into_response();
    }
    (StatusCode::NOT_FOUND, "User not found").into_response()
}

pub async fn delete_user(Path(id): Path<u32>, State(users): State<UserList>) -> impl IntoResponse {
    let mut users = users.lock().unwrap();
    if let Some(index) = users.iter().position(|u| u.id == id) {
        users.remove(index);
        (StatusCode::NO_CONTENT).into_response()
    } else {
        (StatusCode::NOT_FOUND, "User not found").into_response()
    }
}

//Testing api...
pub async fn hello() -> &'static str{
    "My first API"
}