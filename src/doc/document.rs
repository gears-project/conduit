use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::digraph::Digraph;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Document<T> {
    pub id: Uuid,
    pub name: String,
    pub doctype: String,
    pub version: i32,
    pub body: T,
}

impl<T> Document<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Default,
{
    pub fn new(doctype: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "default".to_owned(),
            doctype,
            version: 0,
            body: <T>::default(),
        }
    }

    pub fn change(&mut self) -> i32 {
        self.version += 1;
        self.version
    }
}

pub type RawDocument = Document<String>;

macro_rules! register_doc {
    ($source:ty, $name:ident, $doctype:expr) => {
        // use chrono::NaiveDateTime;
        // use uuid::Uuid;

        pub type $name = Document<$source>;

        impl Default for $name {
            fn default() -> Self {
                Self {
                    id: Uuid::new_v4(),
                    // project_id: crate::util::naming::empty_uuid(),
                    name: "New".to_owned(),
                    doctype: stringify!($doctype).to_owned(),
                    version: 0,
                    // owner: crate::util::naming::empty_uuid(),
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
                    doctype: doc.doctype,
                    name: doc.name,
                    version: doc.version,
                    body: serde_json::to_string(&doc.body).unwrap(),
                }
            }
        }
    };
}

register_doc!(Digraph, DigraphDocument, digraph);

pub enum Doc {
    Digraph(DigraphDocument),
}

#[cfg(test)]
mod test {

    #[test]
    fn test_new_digraph_document() {
        let mut dg = super::DigraphDocument::new("digraph".into());
        let _ = dg.body.add_node(None);
        assert_eq!(dg.body.nodes.len(), 1);
    }
}
