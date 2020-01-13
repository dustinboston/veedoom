#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use wasm_bindgen::prelude::*;
use web_sys::DocumentFragment;

pub mod myers;
pub use myers::shortest_edit;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

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

#[derive(Debug, Deserialize, Serialize)]
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

    pub fn notify(&self, field_changed: FieldChanged, change_type: ChangeType) {
        let window = web_sys::window().expect("no global window exists");
        let document = window.document().expect("should have a document on window");

        match field_changed {
            FieldChanged::Tag => {
                log("TODO: Remove DOM node and recreate");
            }
            FieldChanged::Props => {
                log("TODO: Update all attribtues on DOM node");
            }
            FieldChanged::Children => {
                log("Update children for THIS node");
            }
        }
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

// This is basically create_element, but for Rust tests
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

// Provides a JSX compatible interface for creating nodes
#[wasm_bindgen]
pub fn create_element(tag: String, props_val: &JsValue, children_val: &JsValue) -> Result<JsValue, JsValue> {
    let props: HashMap<String, String> = props_val.into_serde().unwrap();
    let children: Vec<Box<VeeDoom>> = children_val.into_serde().unwrap();

    let key = match props.get("key") {
        Some(k) => k.to_string(),
        None => {
            let start = SystemTime::now();
            let ts = start
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            ts.as_nanos().to_string()
        }
    };

    let node = Box::new(VeeDoom {
        key,
        tag,
        props,
        children,
    });

    Ok(JsValue::from_serde(&node).unwrap())
}

pub fn create_dom_node(node: &Box<VeeDoom>) -> Result<DocumentFragment, JsValue> {
    let fragment = DocumentFragment::new()?;
    let window = web_sys::window().expect("no global window exists");
    let document = window.document().expect("should have a document on window");

    let el = document.create_element(&*node.tag.as_str())?;
    for (key, val) in &node.props {
        el.set_attribute(key, val)?;
    }

    match node.props.get("text") {
        Some(t) => {
            let text = document.create_text_node(&t);
            el.append_with_node_1(&text);
        }
        None => ()
    }

    if &node.children.len() > &0 {
        let children: js_sys::Array = node
            .children
            .iter()
            .map(|c| create_dom_node(c).unwrap())
            .collect();

        el.append_with_node(&children)?;
    }

    fragment.append_with_node_1(&el)?;
    Ok(fragment)
}

#[wasm_bindgen]
pub fn render(node_val: &JsValue, root_id: Option<String>) -> Result<(), JsValue> {
    let node: Box<VeeDoom> = node_val.into_serde().unwrap();

    let dom_node = match create_dom_node(&node) {
        Ok(n) => n,
        Err(_e) => return Err(JsValue::from_str("create dom node error"))
    };

    let window = web_sys::window().expect("no global window exists");
    let document = window.document().expect("should have a document on window");

    match root_id {
        Some(id) => {
            let root = match document.get_element_by_id(id.as_str()) {
                Some(el) => el,
                None => return Err(JsValue::from_str("undefined element")),
            };
            root.append_with_node_1(&dom_node)?;
        }
        None => return Ok(())
    } 
    Ok(())
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
log("YOU HAVE ENTERED...
 ██▒   █▓▓█████ ▓█████ ▓█████▄  ▒█████   ▒█████   ███▄ ▄███▓
▓██░   █▒▓█   ▀ ▓█   ▀ ▒██▀ ██▌▒██▒  ██▒▒██▒  ██▒▓██▒▀█▀ ██▒
 ▓██  █▒░▒███   ▒███   ░██   █▌▒██░  ██▒▒██░  ██▒▓██    ▓██░
  ▒██ █░░▒▓█  ▄ ▒▓█  ▄ ░▓█▄   ▌▒██   ██░▒██   ██░▒██    ▒██ 
   ▒▀█░  ░▒████▒░▒████▒░▒████▓ ░ ████▓▒░░ ████▓▒░▒██▒   ░██▒
   ░ ▐░  ░░ ▒░ ░░░ ▒░ ░ ▒▒▓  ▒ ░ ▒░▒░▒░ ░ ▒░▒░▒░ ░ ▒░   ░  ░
   ░ ░░   ░ ░  ░ ░ ░  ░ ░ ▒  ▒   ░ ▒ ▒░   ░ ▒ ▒░ ░  ░      ░
     ░░     ░      ░    ░ ░  ░ ░ ░ ░ ▒  ░ ░ ░ ▒  ░      ░   
      ░     ░  ░   ░  ░   ░        ░ ░      ░ ░         ░   
     ░                  ░                                   
");
Ok(())
}
