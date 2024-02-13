use axum::{
    routing::get,
    Router,
};
pub fn create_router() -> Router {
    Router::new()
        .route("/users", get(get_users))
     
        .route("/books", get(get_books))
        
}
