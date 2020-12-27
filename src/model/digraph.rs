use std::collections::HashMap;

type Labels = HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum DigraphError {
    IdDoesNotExist,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum DigraphMessage {
    AddNode,
    RemoveNode(i32),
    AddLink(i32, i32),
    RemoveLink(i32),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Digraph {
    pub name: String,
    pub nodes: Vec<Node>,
    pub links: Vec<Link>,
    pub labels: Labels,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Node {
    pub id: i32,
    pub name: String,
    pub labels: Labels,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Link {
    pub id: i32,
    pub name: String,
    pub source: i32,
    pub target: i32,
    pub labels: Labels,
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

    pub fn add_node(&mut self, labels: Option<Labels>) -> Result<(), DigraphError> {
        self.nodes.push(Node {
            id: self.next_id(),
            name: "".into(),
            labels: labels.unwrap_or(Labels::new()),
        });
        Ok(())
    }

    pub fn remove_node(&mut self, id: i32) -> Result<(), DigraphError> {
        if let Some(pos) = self.nodes.iter().position(|e| e.id == id) {
            self.nodes.remove(pos);
            self.links.retain(|e| (e.source != id) && (e.target != id));
            Ok(())
        } else {
            Err(DigraphError::IdDoesNotExist)
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
            Err(DigraphError::IdDoesNotExist)
        } else if !ids.contains(&target) {
            Err(DigraphError::IdDoesNotExist)
        } else {
            self.links.push(Link {
                id: self.next_id(),
                name: "".into(),
                source: source,
                target: target,
                labels: labels.unwrap_or(Labels::new()),
            });
            Ok(())
        }
    }

    pub fn remove_link(&mut self, id: i32) -> Result<(), DigraphError> {
        if let Some(pos) = self.links.iter().position(|e| e.id == id) {
            self.links.remove(pos);
            Ok(())
        } else {
            Err(DigraphError::IdDoesNotExist)
        }
    }

    pub fn message(&mut self, msg: DigraphMessage) -> Result<(), DigraphError> {
        match msg {
            DigraphMessage::AddNode => self.add_node(None),
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

    #[test]
    fn test_add_node() {
        let mut dg = super::Digraph::new();
        let _ = dg.add_node(None);
        assert_eq!(dg.nodes.len(), 1);
    }

    #[test]
    fn test_add_multiple_nodes() {
        let mut dg = super::Digraph::new();
        let _ = dg.add_node(None);
        let _ = dg.add_node(None);
        let _ = dg.add_node(None);
        assert_eq!(dg.nodes.len(), 3);
        assert_eq!(dg.nodes.get(0).unwrap().id, 1);
        assert_eq!(dg.nodes.get(2).unwrap().id, 3);
    }

    #[test]
    fn test_add_link() {
        let mut dg = super::Digraph::new();
        let _ = dg.add_node(None);
        let _ = dg.add_node(None);
        let _ = dg.add_link(1, 2, None);
        assert_eq!(dg.links.len(), 1);
    }

    #[test]
    fn test_remove_node() {
        let mut dg = super::Digraph::new();
        let _ = dg.add_node(None);
        assert_eq!(dg.nodes.len(), 1);

        let _ = dg.remove_node(1);
        assert_eq!(dg.nodes.len(), 0);
    }

    #[test]
    fn test_remove_node_that_does_not_exist() {
        let mut dg = super::Digraph::new();
        let res = dg.remove_node(1);
        assert_eq!(res, Err(super::DigraphError::IdDoesNotExist));
    }

    #[test]
    fn test_remove_link() {
        let mut dg = super::Digraph::new();
        let _ = dg.add_node(None);
        let _ = dg.add_node(None);
        let _ = dg.add_link(1, 2, None);
        assert_eq!(dg.links.len(), 1);
        let res = dg.remove_link(3);
        assert_eq!(res, Ok(()));
    }

    #[test]
    fn test_remove_link_that_does_not_exist() {
        let mut dg = super::Digraph::new();
        let res = dg.remove_link(1);
        assert_eq!(res, Err(super::DigraphError::IdDoesNotExist));
    }

    #[test]
    fn test_remove_node_with_links() {
        let mut dg = super::Digraph::new();
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
        let mut dg = super::Digraph::new();
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
        let mut dg = super::Digraph::new();
        dg.message(super::DigraphMessage::AddNode)
            .expect("Can send a message");
        assert_eq!(dg.nodes.len(), 1);
    }

    #[test]
    fn test_message_remove_node() {
        let mut dg = super::Digraph::new();
        dg.message(super::DigraphMessage::AddNode)
            .expect("Can add a node via message");
        assert_eq!(dg.nodes.len(), 1);
        dg.message(super::DigraphMessage::RemoveNode(1))
            .expect("Can remove a node via message");
        assert_eq!(dg.nodes.len(), 0);
    }

    #[test]
    fn test_message_add_link() {
        let mut dg = super::Digraph::new();
        dg.message(super::DigraphMessage::AddNode)
            .expect("Can add a node via message");
        dg.message(super::DigraphMessage::AddNode)
            .expect("Can add a node via message");
        assert_eq!(dg.nodes.len(), 2);
        dg.message(super::DigraphMessage::AddLink(1, 2))
            .expect("Can add a link via message");
        assert_eq!(dg.links.len(), 1);
    }

    #[test]
    fn test_message_remove_link() {
        let mut dg = super::Digraph::new();
        dg.message(super::DigraphMessage::AddNode)
            .expect("Can add a node via message");
        dg.message(super::DigraphMessage::AddNode)
            .expect("Can add a node via message");
        assert_eq!(dg.nodes.len(), 2);
        dg.message(super::DigraphMessage::AddLink(1, 2))
            .expect("Can add a link via message");
        assert_eq!(dg.links.len(), 1);
        dg.message(super::DigraphMessage::RemoveLink(3))
            .expect("Can remove a link via message");
        assert_eq!(dg.links.len(), 0);
        assert_eq!(dg.nodes.len(), 2);
    }
}
