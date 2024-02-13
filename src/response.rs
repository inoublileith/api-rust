use crate::model::User;
use crate::model::Book;
use serde::Serialize;


#[derive(Serialize, Debug)]
pub struct UsersListResponse {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct BooksListResponse {
    pub name: String,
 
}