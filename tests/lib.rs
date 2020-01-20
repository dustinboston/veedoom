//! Test suite for node 
#![cfg(target_arch = "wasm32")]

#[macro_use]
extern crate serde_derive;

extern crate js_sys;
extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;

use veedoom::v;
use veedoom::node::Node;

#[wasm_bindgen_test]
fn should_make_node() {

    let js_node = v(
        "blink".to_string(),
        &js_sys::Map::new(),
        &js_sys::Array::new()
    ).expect("JsValue could not be unwrapped");

    let node: Node = js_node.into_serde().unwrap();
    assert_eq!(node.tag, "blink".to_string());
}
