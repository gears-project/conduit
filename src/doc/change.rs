use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Change {
    pub id: i32,
    pub document_id: Uuid,
    pub version: i32,
    pub forward: Value,
    pub reverse: Value,
}
