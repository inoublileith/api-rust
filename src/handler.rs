
//import axum web framework
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

//import U
use uuid::Uuid;

use crate::{
    model::{User,Book},
    response::{ UsersListResponse,BooksListResponse},
};

 async fn todos_list_handler(
    opts: Option<Query<QueryOptions>>
) -> impl IntoResponse {
    

    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let resp = match fetch_users().await {
        Ok(resp) => resp,
        Err(_) => return "Failed to fetch users.".to_string(),
    };

    // Read the response body
    match resp.text().await {
        Ok(body) => body,
        Err(_) => "Failed to read response body.".to_string(),
    }

    let json_response = UserListResponse {
        status: "success".to_string(),
        results: todos.len(),
        todos,
    };

    Json(json_response)
}