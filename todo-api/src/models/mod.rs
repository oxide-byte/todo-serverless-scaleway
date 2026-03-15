use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar")]
pub enum Status {
    Active,
    Completed
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct NewTodo {
    pub title: String,
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct EditTodo {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: Status,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo {
    pub id: Uuid,
    pub owner: String,
    pub title: String,
    pub description: String,
    pub status: Status,
    pub created: DateTime<Utc>,
    pub updated: Option<DateTime<Utc>>
}

impl Todo {

    pub fn generate_id(&mut self) {
        self.id = Uuid::new_v4();
    }

    pub fn new(title: String, description: String) -> Todo {
        Todo {
            id: Uuid::new_v4(),
            owner: "default_owner".to_string(),
            title,
            description,
            status: Status::Active,
            created: Utc::now(),
            updated: None
        }
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;