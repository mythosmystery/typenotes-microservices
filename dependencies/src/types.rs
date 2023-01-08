use serde::{Serialize, Deserialize};

use crate::models::note::NoteType;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub password_hash: String,
    pub invite_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNote {
    pub title: String,
    pub body: String,
    pub user: String,
    pub note_type: NoteType,
    pub image: Option<String>,
}