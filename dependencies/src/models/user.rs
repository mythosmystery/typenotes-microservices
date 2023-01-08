use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_code: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime>,
    pub notes: Vec<ObjectId>,
    #[serde(rename = "type")]
    pub user_type: UserType,
}

impl User {
    pub fn new(name: String, email: String, phone: String, password: String, invite_code: String) -> Self {
        Self {
            id: Some(ObjectId::new()),
            name,
            email,
            phone,
            password,
            invite_code: Some(invite_code),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            deleted_at: None,
            notes: Vec::new(),
            user_type: UserType::User,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum UserType {
    #[default]
    Guest,
    User,
    Admin,
    Dev,
}
