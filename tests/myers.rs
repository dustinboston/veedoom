//! Test suite for node 

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use veedoom::myers::shortest_edit;

#[wasm_bindgen_test]
fn should_diff_insert() {
    let a: Vec<String> = vec![];
    let b: Vec<String> = vec!["b".to_string()];
    let path = shortest_edit(&a, &b);
    assert_eq!(path.len(), 1);
}

#[wasm_bindgen_test]
fn should_diff_replace() {
    let a: Vec<String> = vec!["a".to_string()];
    let b: Vec<String> = vec!["b".to_string()];
    let path = shortest_edit(&a, &b);
    assert_eq!(path.len(), 2);
}

#[wasm_bindgen_test]
fn should_diff_delete() {
    let a: Vec<String> = vec!["a".to_string()];
    let b: Vec<String> = vec![];
    let path = shortest_edit(&a, &b);
    assert_eq!(path.len(), 1);
}
