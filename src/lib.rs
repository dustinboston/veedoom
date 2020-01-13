use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub mod myers;
pub use myers::shortest_edit;

// TODO: The Diffler

#[derive(Debug)]
pub enum FieldChanged {
    Tag,
    Props,
    Children,
}

#[derive(Debug)]
pub enum ChangeType {
    Created,
    Updated,
    Deleted,
    Equal,
}

#[derive(Debug)]
pub struct VeeDoom {
    pub key: String,
    pub tag: String,
    pub props: HashMap<String, String>,
    pub children: Vec<Box<VeeDoom>>,
}

impl VeeDoom {
    pub fn update(&mut self, other: Box<VeeDoom>) {
        // if tag changed, replace this veedoom with the new one
        if self.tag != other.tag {
            self.notify(FieldChanged::Tag, ChangeType::Updated);
            *self = VeeDoom { ..*other };
            return;
        }

        // if props are different, replace self with new props outright
        // TODO: check props for changes
        // TODO: check the style prop specifically
        if self.props != other.props {
            self.notify(FieldChanged::Props, ChangeType::Updated);
            self.props = other.props;
        }

        // Check children for changes
        let a: Vec<String> = self.children.iter().map(|c| c.key.to_string()).collect();
        let b: Vec<String> = other.children.iter().map(|c| c.key.to_string()).collect();

        let path = shortest_edit(&a, &b);
        if path.len() > 0 {
            for v in path.iter().rev() {
                let (prev_x, prev_y, x, y) = v;
                if x == prev_x {
                    // insert
                    let at = *y as isize - 1;
                    self.children
                        .insert(at as usize, other.children[*prev_y as usize].clone());
                    self.notify(FieldChanged::Children, ChangeType::Created);
                } else if y == prev_y {
                    // delete
                    let _removed_key = self.children.remove(*y as usize);
                    self.notify(FieldChanged::Children, ChangeType::Deleted);
                } else {
                    // equal, recurse!
                    self.notify(FieldChanged::Children, ChangeType::Equal);
                    let at_cur = *x as isize - 1;
                    let at_new = *y as isize - 1;
                    self.children[at_cur as usize].update(
                        other.children[at_new as usize].clone()
                    );
                }
            }
        } else {
            // No changes, recurse children
            for i in 0..self.children.len() {
                self.children[i].update(
                    other.children[i].clone()
                );
            }
        }
    }

    pub fn notify(&self, field: FieldChanged, change_type: ChangeType) {
        println!("[notify] {:?} {:?}", field, change_type);
    }

    pub fn log(&self, msg: String) {
        println!("[log] {}", msg);
    }
}

impl Clone for VeeDoom {
    fn clone(&self) -> VeeDoom {
        VeeDoom {
            key: self.key.to_string(),
            tag: self.tag.to_string(),
            props: self.props.clone(),
            children: self.children.clone(),
        }
    }
}

impl PartialEq for VeeDoom {
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag && self.props == other.props && *self.children == *other.children
    }
}

pub fn n(tag: &str, props: Vec<(&str, &str)>, children: Vec<Box<VeeDoom>>) -> Box<VeeDoom> {
    let mut prop_map = HashMap::new();
    for pair in props.iter() {
        prop_map.insert(pair.0.to_string(), pair.1.to_string());
    }

    let key = match prop_map.get("key") {
        Some(k) => k.to_string(),
        None => {
            let start = SystemTime::now();
            let ts = start
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            ts.as_nanos().to_string()
        }
    };

    Box::new(VeeDoom {
        key,
        tag: String::from(tag),
        props: prop_map,
        children,
    })
}
