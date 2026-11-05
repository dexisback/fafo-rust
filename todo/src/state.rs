//creating the shared application state


use std::{
    collections::HashMap,
    sync::Arc,
};


use tokio::sync::RwLock;
use uuid::Uuid;



use crate::models::todo::Todo;

#[derive(Clone)]
pub struct AppState {
    pub todos: Arc<RwLock<HashMap<Uuid, Todo>>>
}

impl AppState {
    pub fn new() -> Self {
        Self {
            todos: Arc::new(RwLock::new(HashMap::new())),
        }
    }



}