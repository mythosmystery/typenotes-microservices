use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub body: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime>,
    pub user: ObjectId,
    #[serde(rename = "type")]
    pub note_type: NoteType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}

impl Note {
    pub fn new(title: String, body: String, user: ObjectId, note_type: NoteType, image: Option<String>) -> Self {
        Self {
            id: Some(ObjectId::new()),
            title,
            body,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            deleted_at: None,
            user,
            note_type,
            image,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum NoteType {
    #[default]
    Text,
    Image,
    Markdown,
    Html,
}
