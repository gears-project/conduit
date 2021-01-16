use std::collections::HashMap;
use std::{error, fmt};

type Labels = HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum DigraphError {
    IdDoesNotExist(i32),
}

impl fmt::Display for DigraphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DigraphError::IdDoesNotExist(e) => write!(f, "id does not exist : {}", e),
        }
    }
}

impl error::Error for DigraphError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum DigraphMessage {
    AddNode(NodeAttributes),
    UpdateNode(i32, NodeAttributes),
    RemoveNode(i32),
    AddLink(i32, i32),
    RemoveLink(i32),
}

#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Digraph {
    pub name: String,
    pub nodes: Vec<Node>,
    pub links: Vec<Link>,
    pub labels: Labels,
}

#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Node {
    pub id: i32,
    pub name: String,
    pub labels: Labels,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NodeAttributes {
    pub name: Option<String>,
    pub labels: Option<Labels>,
}

impl Node {
    pub fn update(&mut self, attrs: NodeAttributes) -> () {
        if let Some(name) = attrs.name {
            self.name = name;
        }

        if let Some(labels) = attrs.labels {
            for (key, value) in &labels {
                let _ = self.labels.insert(key.to_string(), value.to_string());
            }
        }
    }
}

impl Default for NodeAttributes {
    fn default() -> Self {
        Self {
            name: Some("".to_string()),
            labels: Some(Labels::new()),
        }
    }
}

#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Link {
    pub id: i32,
    pub name: String,
    pub source: i32,
    pub target: i32,
    pub labels: Labels,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LinkAttributes {
    pub name: Option<String>,
    pub source: Option<i32>,
    pub target: Option<i32>,
    pub labels: Option<Labels>,
}

impl Link {
    pub fn update(&mut self, attrs: LinkAttributes) -> () {
        if let Some(name) = attrs.name {
            self.name = name;
        }

        if let Some(source) = attrs.source {
            self.source = source;
        }

        if let Some(target) = attrs.target {
            self.target = target;
        }

        if let Some(labels) = attrs.labels {
            for (key, value) in &labels {
                let _ = self.labels.insert(key.to_string(), value.to_string());
            }
        }
    }
}

impl Default for Digraph {
    fn default() -> Self {
        Self::new()
    }
}

impl Digraph {
    pub fn new() -> Self {
        Self {
            name: "".into(),
            nodes: Vec::<Node>::new(),
            links: Vec::<Link>::new(),
            labels: Labels::new(),
        }
    }

    fn node_ids(&self) -> Vec<i32> {
        let ids: Vec<i32> = self.nodes.iter().map(|e| e.id).collect();
        ids
    }

    fn link_ids(&self) -> Vec<i32> {
        let ids: Vec<i32> = self.links.iter().map(|e| e.id).collect();
        ids
    }

    fn all_ids(&self) -> Vec<i32> {
        let mut ids = self.node_ids();
        ids.append(&mut self.link_ids());
        ids
    }

    fn highest_id(&self) -> i32 {
        if let Some(id) = self.all_ids().iter().max() {
            *id
        } else {
            0
        }
    }

    fn next_id(&self) -> i32 {
        self.highest_id() + 1
    }

    pub fn add_node(&mut self, attrs: Option<NodeAttributes>) -> Result<(), DigraphError> {
        let attrs = attrs.unwrap_or_default();

        self.nodes.push(Node {
            id: self.next_id(),
            name: attrs.name.unwrap_or("".into()),
            labels: attrs.labels.unwrap_or(Labels::new()),
        });
        Ok(())
    }

    pub fn update_node(&mut self, id: i32, attrs: NodeAttributes) -> Result<(), DigraphError> {
        if let Some(pos) = self.nodes.iter().position(|e| e.id == id) {
            let node = self
                .nodes
                .get_mut(pos)
                .expect("Node to exist at this position");
            node.update(attrs);
            Ok(())
        } else {
            Err(DigraphError::IdDoesNotExist(id))
        }
    }

    pub fn remove_node(&mut self, id: i32) -> Result<(), DigraphError> {
        if let Some(pos) = self.nodes.iter().position(|e| e.id == id) {
            self.nodes.remove(pos);
            self.links.retain(|e| (e.source != id) && (e.target != id));
            Ok(())
        } else {
            Err(DigraphError::IdDoesNotExist(id))
        }
    }

    pub fn add_link(
        &mut self,
        source: i32,
        target: i32,
        labels: Option<Labels>,
    ) -> Result<(), DigraphError> {
        let ids = self.node_ids();
        if !ids.contains(&source) {
            Err(DigraphError::IdDoesNotExist(source))
        } else if !ids.contains(&target) {
            Err(DigraphError::IdDoesNotExist(target))
        } else {
            self.links.push(Link {
                id: self.next_id(),
                name: "".into(),
                source,
                target,
                labels: labels.unwrap_or_default(),
            });
            Ok(())
        }
    }

    pub fn remove_link(&mut self, id: i32) -> Result<(), DigraphError> {
        if let Some(pos) = self.links.iter().position(|e| e.id == id) {
            self.links.remove(pos);
            Ok(())
        } else {
            Err(DigraphError::IdDoesNotExist(id))
        }
    }

    pub fn message(&mut self, msg: DigraphMessage) -> Result<(), DigraphError> {
        match msg {
            DigraphMessage::AddNode(attrs) => self.add_node(Some(attrs)),
            DigraphMessage::UpdateNode(id, attrs) => self.update_node(id, attrs),
            DigraphMessage::RemoveNode(id) => self.remove_node(id),
            DigraphMessage::AddLink(source_id, target_id) => {
                self.add_link(source_id, target_id, None)
            }
            DigraphMessage::RemoveLink(id) => self.remove_link(id),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_add_node() {
        let mut dg = Digraph::new();
        let _ = dg.add_node(None);
        assert_eq!(dg.nodes.len(), 1);
    }

    #[test]
    fn test_add_multiple_nodes() {
        let mut dg = Digraph::new();
        let _ = dg.add_node(None);
        let _ = dg.add_node(None);
        let _ = dg.add_node(None);
        assert_eq!(dg.nodes.len(), 3);
        assert_eq!(dg.nodes.get(0).unwrap().id, 1);
        assert_eq!(dg.nodes.get(2).unwrap().id, 3);
    }

    #[test]
    fn test_add_link() {
        let mut dg = Digraph::new();
        let _ = dg.add_node(None);
        let _ = dg.add_node(None);
        let _ = dg.add_link(1, 2, None);
        assert_eq!(dg.links.len(), 1);
    }

    #[test]
    fn test_remove_node() {
        let mut dg = Digraph::new();
        let _ = dg.add_node(None);
        assert_eq!(dg.nodes.len(), 1);

        let _ = dg.remove_node(1);
        assert_eq!(dg.nodes.len(), 0);
    }

    #[test]
    fn test_id_generation() {
        let mut dg = Digraph::new();
        let _ = dg.add_node(None);
        let _ = dg.add_node(None);
        let _ = dg.add_node(None);

        assert_eq!(dg.nodes[0].id, 1);
        assert_eq!(dg.nodes[1].id, 2);
        assert_eq!(dg.nodes[2].id, 3);

        let _ = dg.remove_node(2);
        assert_eq!(dg.nodes.len(), 2);

        let _ = dg.add_node(None);

        assert_eq!(dg.nodes[0].id, 1);
        assert_eq!(dg.nodes[1].id, 3);
        assert_eq!(dg.nodes[2].id, 4);

    }

    #[test]
    fn test_remove_node_that_does_not_exist() {
        let mut dg = Digraph::new();
        let res = dg.remove_node(1);
        assert_eq!(res, Err(DigraphError::IdDoesNotExist(1)));
    }

    #[test]
    fn test_remove_link() {
        let mut dg = Digraph::new();
        let _ = dg.add_node(None);
        let _ = dg.add_node(None);
        let _ = dg.add_link(1, 2, None);
        assert_eq!(dg.links.len(), 1);
        let res = dg.remove_link(3);
        assert_eq!(res, Ok(()));
    }

    #[test]
    fn test_remove_link_that_does_not_exist() {
        let mut dg = Digraph::new();
        let res = dg.remove_link(1);
        assert_eq!(res, Err(DigraphError::IdDoesNotExist(1)));
    }

    #[test]
    fn test_remove_node_with_links() {
        let mut dg = Digraph::new();
        let _ = dg.add_node(None);
        let _ = dg.add_node(None);
        let _ = dg.add_link(1, 2, None);
        assert_eq!(dg.links.len(), 1);

        let _ = dg.remove_node(1);
        assert_eq!(dg.links.len(), 0);
        assert_eq!(dg.nodes.len(), 1);
    }

    #[test]
    fn test_serialization() {
        let mut dg = Digraph::new();
        let _ = dg.add_node(None);
        let _ = dg.add_node(None);
        let _ = dg.add_link(1, 2, None);
        assert_eq!(dg.links.len(), 1);

        assert_eq!(
            dg,
            serde_json::from_str(&serde_json::to_string(&dg).unwrap()).unwrap()
        );
    }

    #[test]
    fn test_message_add_node() {
        let mut dg = Digraph::new();
        dg.message(DigraphMessage::AddNode(NodeAttributes::default()))
            .expect("Can send a message");
        assert_eq!(dg.nodes.len(), 1);
    }

    #[test]
    fn test_message_update_node() {
        let mut dg = Digraph::new();
        dg.message(DigraphMessage::AddNode(NodeAttributes::default()))
            .expect("Can send a message");
        dg.message(DigraphMessage::UpdateNode(
            1,
            NodeAttributes {
                name: Some("Test 1".into()),
                labels: None,
            },
        ))
        .expect("Can send a message");
        assert_eq!(dg.nodes.len(), 1);
        assert_eq!(dg.nodes.get(0).unwrap().name, "Test 1".to_string());
    }

    #[test]
    fn test_message_remove_node() {
        let mut dg = Digraph::new();
        dg.message(DigraphMessage::AddNode(NodeAttributes::default()))
            .expect("Can add a node via message");
        assert_eq!(dg.nodes.len(), 1);
        dg.message(DigraphMessage::RemoveNode(1))
            .expect("Can remove a node via message");
        assert_eq!(dg.nodes.len(), 0);
    }

    #[test]
    fn test_message_add_link() {
        let mut dg = Digraph::new();
        dg.message(DigraphMessage::AddNode(NodeAttributes::default()))
            .expect("Can add a node via message");
        dg.message(DigraphMessage::AddNode(NodeAttributes::default()))
            .expect("Can add a node via message");
        assert_eq!(dg.nodes.len(), 2);
        dg.message(DigraphMessage::AddLink(1, 2))
            .expect("Can add a link via message");
        assert_eq!(dg.links.len(), 1);
    }

    #[test]
    fn test_message_remove_link() {
        let mut dg = Digraph::new();
        dg.message(DigraphMessage::AddNode(NodeAttributes::default()))
            .expect("Can add a node via message");
        dg.message(DigraphMessage::AddNode(NodeAttributes::default()))
            .expect("Can add a node via message");
        assert_eq!(dg.nodes.len(), 2);
        dg.message(DigraphMessage::AddLink(1, 2))
            .expect("Can add a link via message");
        assert_eq!(dg.links.len(), 1);
        dg.message(DigraphMessage::RemoveLink(3))
            .expect("Can remove a link via message");
        assert_eq!(dg.links.len(), 0);
        assert_eq!(dg.nodes.len(), 2);
    }

}
