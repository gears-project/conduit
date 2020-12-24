use std::collections::HashMap;

type Labels = HashMap<String, String>;

pub enum DigraphError {
    DuplicateId,
    IdDoesNotExist,
}

pub struct Digraph {
    pub nodes: Vec<Node>,
    pub links: Vec<Link>,
    pub labels: Labels,
}

#[derive(Clone)]
pub struct Node {
    pub id: i32,
    pub labels: Labels,
}

pub struct Link {
    pub id: i32,
    pub source: i32,
    pub target: i32,
    pub labels: Labels,
}

impl Digraph {

    pub fn new() -> Self {
        Self {
            nodes: Vec::<Node>::new(),
            links: Vec::<Link>::new(),
            labels: Labels::new(),
        }
    }

    fn node_ids(&self) -> Vec<i32> {
        let ids : Vec<i32> = self.nodes
            .iter()
            .map(|e| e.id )
            .collect();
        ids
    }

    fn link_ids(&self) -> Vec<i32> {
        let ids : Vec<i32> = self.links
            .iter()
            .map(|e| e.id )
            .collect();
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
        let node = Node {
            id: self.next_id(),
            labels: labels.unwrap_or(Labels::new()),
        };
        self.nodes.push(node);
        Ok(())
    }

    pub fn add_link(&mut self, source: i32, target: i32, labels: Option<Labels>) -> Result<(), DigraphError> {
        let ids = self.all_ids();
        if !ids.contains(&source) {
            Err(DigraphError::IdDoesNotExist)
        } else if !ids.contains(&target) {
            Err(DigraphError::IdDoesNotExist)
        } else {
            self.links.push(Link {
                id: self.next_id(),
                source: source,
                target: target,
                labels: labels.unwrap_or(Labels::new()),
            });
            Ok(())
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

}
