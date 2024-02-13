use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: Option<String>,
    pub name: String,
    pub email: String,
    pub password: Option<bool>,
    pub createdAt: Option<DateTime<Utc>>,
  
}


#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Book {
    pub id: Option<String>,
    pub name: String,
    pub createdAt: Option<DateTime<Utc>>,
  
}
