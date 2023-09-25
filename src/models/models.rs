use mongodb::bson::oid::ObjectID;
use serde::{Deserialize, Serilalize};
use chrono::prelude::*;

#[derive(Debug, Serilalize, Deserialize, Clone)]
pub struct User {
    #[serde(rename="_id", skip_serializing_if = "Option::is_none")]
    pub id : Option<ObjectID>,
    pub name : String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginSchema {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serilalize, Deserialize, Clone )]
pub struct Todo {
    #[serde(rename="_id", skip_serializing_if = "Option::is_none")]
    pub todo_id : Option<ObjectID>,
    pub user: User,
    pub task: String,
    pub time: Utc::now(),
}