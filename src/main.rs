use axum::{
    routing::get,
    Router,
    
};
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use lazy_static::lazy_static;
use postgrest::Postgrest;
use reqwest::Error;
use tower_http::cors::CorsLayer;
lazy_static! {
    static ref CLIENT: Postgrest = {
        let client = Postgrest::new("https://figcmvgsmtwnayohdiik.supabase.co/rest/v1/")
            .insert_header(
                "apikey",
                "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImZpZ2NtdmdzbXR3bmF5b2hkaWlrIiwicm9sZSI6ImFub24iLCJpYXQiOjE3MDQyODIwODQsImV4cCI6MjAxOTg1ODA4NH0.rB2UqZPIacGc5mMIPeZYBAxsNJfaxITXFF0IikFo_as",
            );
        client
    };
}

pub fn create_router() -> Router {
    Router::new()
        .route("/users", get(get_users))
        
        .route("/clients", get(get_books))
        
}





async fn get_users() -> String {
    // Fetch data from the "users" table
    let resp = match fetch_users().await {
        Ok(resp) => resp,
        Err(_) => return "Failed to fetch users.".to_string(),
    };

    // Read the response body
    match resp.text().await {
        Ok(body) => body,
        Err(_) => "Failed to read response body.".to_string(),
    }
}

async fn fetch_users() -> Result<reqwest::Response, Error> {
    CLIENT.from("users").select("*").execute().await
}

async fn get_books() -> String {
    // Fetch data from the "books" table
    let resp = match fetch_books().await {
        Ok(resp) => resp,
        Err(_) => return "Failed to fetch books.".to_string(),
    };

    // Read the response body
    match resp.text().await {
        Ok(body) => body,
        Err(_) => "Failed to read response body.".to_string(),
    }
}

async fn fetch_books() -> Result<reqwest::Response, Error> {
    CLIENT.from("clients").select("*").execute().await
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cors = CorsLayer::new()
    .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
    .allow_credentials(true)
    .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

let app = create_router().layer(cors);
    println!("🚀 Server started successfully");
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
