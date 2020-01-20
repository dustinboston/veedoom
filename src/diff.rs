use crate::node::Node;

#[derive(Debug, Deserialize, Serialize)]
pub enum FieldChanged {
    Tag,
    Props,
    Children,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ChangeType {
    Created,
    Updated,
    Deleted,
    Equal,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Diff {
    // parent_key?
    pub key: String,
    pub field: FieldChanged,
    pub change: ChangeType,
    pub old_val: Option<String>,
    pub old_pos: Option<usize>,
    pub new_val: Option<String>,
    pub new_pos: Option<usize>,
    pub old_node: Option<Node>,
    pub new_node: Option<Node>,
}
