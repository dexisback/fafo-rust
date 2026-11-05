//defining the todo model


use serde::{Deserialize, Serialize};
use uuid::Uuid;

//todo has id, title and iscompleted and body
//create todo me bas title and body
#[derive(Debug, Clone, Serialize)]
pub struct Todo {
    pub id: Uuid, 
    pub title: String,
    pub body: String,
    pub is_completed: bool,
}


#[derive(Debug, Deserialize)]

pub struct CreateTodo {
    pub title: String,
    pub body: String,
    pub is_completed: bool,
}


//serialise and deserialise are seder deivated. they
//they generate the implementations needed to convert these structs to and from JSON



