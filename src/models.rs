use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Post {
    title: String,
    body: String,
    author: String,
    datetime: DateTime<Utc>,
    uuid: Uuid,
}

impl Post {
    pub fn new(title: &str, body: &str, author: &str) -> Post {
        Post {
            title: title.to_string(),
            body: body.to_string(),
            author: author.to_string(),
            datetime: chrono::offset::Utc::now(),
            uuid: Uuid::new_v4(),
        }
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PostInput {
    pub title: String,
    pub body: String,
    pub author: String,
}
