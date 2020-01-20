#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use wasm_bindgen::prelude::*;

pub mod diff;
pub mod myers;
pub mod node;

pub use myers::shortest_edit;
pub use diff::{FieldChanged, ChangeType, Diff};
pub use node::Node;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(module = "/dom.js")]
extern "C" {
    #[wasm_bindgen(js_name = onChange)]
    fn on_change(diff: &JsValue);
}

// Provides a near JSX compatible interface for creating nodes
#[wasm_bindgen]
pub fn v(tag: String, props_val: &JsValue, children_val: &JsValue) -> Result<JsValue, JsValue> {
    let props: HashMap<String, String> = props_val.into_serde().unwrap();
    let children: Vec<Box<Node>> = children_val.into_serde().unwrap();

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

    let node = Box::new(Node {
        key,
        tag,
        props,
        children,
    });

    Ok(JsValue::from_serde(&node).unwrap())
}

#[wasm_bindgen]
pub fn update(current_val: &JsValue, other_val: &JsValue) -> Result<JsValue, JsValue> {
    let mut current: Node = current_val.into_serde().unwrap();
    let other: Node = other_val.into_serde().unwrap();

    // if tag changed, replace this veedoom with the new one
    if current.tag != other.tag {
        notify(Diff {
            key: current.key.to_string(),
            field: FieldChanged::Tag,
            change: ChangeType::Updated,
            old_val: None, // Some(current.tag.to_string()),
            new_val: None, // Some(other.tag.to_string()),
            old_pos: None,
            new_pos: None,
            old_node: Some(current.clone()),
            new_node: Some(other.clone())
        });
        current = Node { ..other };
        return Ok(JsValue::from_serde(&current).unwrap())
    }

    // if props are different, replace current with new props outright
    // TODO: check props for changes individually
    // TODO: check the style prop specifically
    if current.props != other.props {
        notify(Diff {
            key: current.key.to_string(),
            field: FieldChanged::Props,
            change: ChangeType::Updated,
            old_val: Some("TODO".to_string()),
            new_val: Some("TODO".to_string()),
            old_pos: None,
            new_pos: None,
            old_node: Some(current.clone()),
            new_node: Some(other.clone())
        });
        current.props = other.props;
    }

    // Check children for changes
    let a: Vec<String> = current.children.iter().map(|c| c.key.to_string()).collect();
    let b: Vec<String> = other.children.iter().map(|c| c.key.to_string()).collect();

    let path = shortest_edit(&a, &b);
    if path.len() > 0 {
        for v in path.iter().rev() {
            let (prev_x, prev_y, x, y) = v;
            if x == prev_x {
                // insert
                let at = *y as isize - 1;
                current.children
                    .insert(at as usize, other.children[*prev_y as usize].clone());
                notify(Diff {
                    key: current.key.to_string(),
                    field: FieldChanged::Children,
                    change: ChangeType::Created,
                    old_val: None,
                    new_val: None,
                    old_pos: None,
                    new_pos: Some(at.clone() as usize),
                    old_node: None,
                    new_node: Some(*other.children[*prev_y as usize].clone()),
                });
            } else if y == prev_y {
                // delete
                let at = *y as isize;
                let removed = current.children.remove(at as usize);
                notify(Diff {
                    key: removed.key.to_string(),
                    field: FieldChanged::Children,
                    change: ChangeType::Deleted,
                    old_val: None,
                    new_val: None,
                    old_pos: Some(at as usize),
                    new_pos: None,
                    old_node: None,
                    new_node: None,
                });
            } else {
                // equal, recurse!
                let at_cur = *x as isize - 1;
                let at_new = *y as isize - 1;
                update(
                    &JsValue::from_serde(&current.children[at_cur as usize]).unwrap(),
                    &JsValue::from_serde(&other.children[at_new as usize].clone()).unwrap()
                )?;
            }
        }
    } else {
        // No changes, recurse children
        for i in 0..current.children.len() {
            update(
                &JsValue::from_serde(&current.children[i]).unwrap(),
                &JsValue::from_serde(&other.children[i].clone()).unwrap()
            )?;
        }
    }

    Ok(JsValue::from_serde(&current).unwrap())
}

// This could be used for broadcasting events to a DOM
pub fn notify(diff: Diff) {
    on_change(&JsValue::from_serde(&diff).unwrap());
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

                    - enter a command -
");

    Ok(())
}
