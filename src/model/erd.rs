
use std::{fmt, error};
use crate::model::digraph::Digraph;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ERD(Digraph);

impl Default for ERD {
    fn default() -> Self {
        Self(Digraph::default())
    }
}

#[derive(Debug, PartialEq)]
pub enum DomainError {
    EntityDoesNotExist(i32),
    AttributeDoesNotExist(i32, i32),
    ReferenceDoesNotExist(i32, i32),
    EntityAlreadyExists(String),
    AttributeAlreadyExists(i32, String),
    ReferenceAlreadyExists(i32, String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DomainError::EntityDoesNotExist(e) => write!(f, "id:{}", e),
            DomainError::AttributeDoesNotExist(e, a) => write!(f, "id:{}/id:{}", e, a),
            DomainError::ReferenceDoesNotExist(e, r) => write!(f, "id:{}/id:{}", e, r),
            DomainError::EntityAlreadyExists(e) => write!(f, "{}", e),
            DomainError::AttributeAlreadyExists(e, a) => write!(f, "id:{}/{}", e, a),
            DomainError::ReferenceAlreadyExists(e, r) => write!(f, "id:{}/{}", e, r),
        }
    }
}

impl error::Error for DomainError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

