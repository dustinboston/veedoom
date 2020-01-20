use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Node {
    pub key: String,
    pub tag: String,
    pub props: HashMap<String, String>,
    pub children: Vec<Box<Node>>,
}

impl Clone for Node {
    fn clone(&self) -> Node {
        Node {
            key: self.key.to_string(),
            tag: self.tag.to_string(),
            props: self.props.clone(),
            children: self.children.clone(),
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag && self.props == other.props && *self.children == *other.children
    }
}


