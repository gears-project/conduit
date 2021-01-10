use serde::{Deserialize, Serialize};
use uuid::Uuid;
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Change {
    pub id: i32,
    pub document_id: Uuid,
    pub version: i32,
    pub forward: Value,
    pub reverse: Value,
}

