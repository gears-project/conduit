use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
pub type DigraphDocument = Document<crate::model::digraph::Digraph>;

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
