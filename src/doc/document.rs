use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::project::Project;
use crate::model::digraph::Digraph;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Document<T> {
    pub id: Uuid,
    pub project_id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub doctype: String,
    pub version: i32,
    pub body: T,
}

impl<T> Document<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Default,
{
    pub fn change(&mut self) -> i32 {
        self.version += 1;
        self.version
    }
}

pub type RawDocument = Document<serde_json::Value>;

macro_rules! register_doc {
    ($source:ty, $name:ident, $doctype:expr) => {
        // use chrono::NaiveDateTime;

        pub type $name = Document<$source>;

        impl $name {
            pub fn create(project: &Project) -> Self {
                let mut doc = Self::default();
                doc.project_id = project.id.clone();
                doc.owner_id = project.owner_id.clone();
                doc
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    id: Uuid::new_v4(),
                    project_id: crate::util::naming::empty_uuid(),
                    owner_id: crate::util::naming::empty_uuid(),
                    name: "New".to_owned(),
                    doctype: stringify!($doctype).to_owned(),
                    version: 0,
                    // created_at: NaiveDateTime::from_timestamp(0, 0),
                    // updated_at: NaiveDateTime::from_timestamp(0, 0),
                    body: <$source>::default(),
                }
            }
        }

        impl From<$name> for RawDocument {
            fn from(doc: $name) -> RawDocument {
                RawDocument {
                    id: doc.id,
                    project_id: doc.project_id,
                    owner_id: doc.owner_id,
                    doctype: doc.doctype,
                    name: doc.name,
                    version: doc.version,
                    body: serde_json::to_value(&doc.body).expect("Document to be serializable"),
                }
            }
        }

        impl From<RawDocument> for $name {
            fn from(doc: RawDocument) -> $name {
                $name {
                    id: doc.id,
                    project_id: doc.project_id,
                    owner_id: doc.owner_id,
                    doctype: doc.doctype,
                    name: doc.name,
                    version: doc.version,
                    body: serde_json::from_value(doc.body)
                        .expect("Serialized data to be deserializable"),
                }
            }
        }

        impl From<&RawDocument> for $name {
            fn from(doc: &RawDocument) -> $name {
                $name {
                    id: doc.id,
                    project_id: doc.project_id,
                    owner_id: doc.owner_id,
                    doctype: doc.doctype.clone(),
                    name: doc.name.clone(),
                    version: doc.version,
                    body: serde_json::from_value(doc.body.clone())
                        .expect("Serialized data to be deserializable"),
                }
            }
        }
    };
}

pub enum DocType {
    Digraph,
}

impl DocType {
    pub fn to_string(&self) -> String {
        match self {
            Self::Digraph => "digraph".to_string(),
        }
    }
}

register_doc!(Digraph, DigraphDocument, digraph);

/*
pub fn raw_doc_to_typed(doc: &RawDocument) -> Result<Doc, String> {
    match doc.doctype.as_ref() {
        "digraph" => Ok(Doc::Digraph(doc.into())),
        _ => Err("Bad document".to_string()),
    }
}
*/

#[cfg(test)]
mod test {

    #[test]
    fn test_new_digraph_document() {
        let mut dg = super::DigraphDocument::default();
        let _ = dg.body.add_node(None);
        assert_eq!(dg.body.nodes.len(), 1);
    }
}
