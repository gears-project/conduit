use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Project {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub version: i32,
    pub name: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ProjectField {
    Name,
    CreatedAt,
    UpdatedAt,
    Version,
}

impl Project {
    pub fn new(owner_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            owner_id,
            name: "New Project".to_string(),
            version: 0,
            body: "".into(),
        }
    }
}
