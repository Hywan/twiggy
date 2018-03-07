#![cfg(target_arch = "wasm32")]
#![feature(proc_macro)]

extern crate wasm_bindgen;

extern crate svelte_analyze as analyze;
extern crate svelte_ir as ir;
extern crate svelte_opt as opt;
extern crate svelte_parser as parser;
extern crate svelte_traits as traits;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Items {
    items: ir::Items,
}

#[wasm_bindgen]
impl Items {
    pub fn parse(data: &[u8]) -> Items {
        let items = parser::parse(data).unwrap();
        Items { items }
    }

    pub fn top(&mut self, options: &opt::Top) -> String {
        let top = analyze::top(&mut self.items, options).unwrap();
        let mut buf = Vec::new();
        top.emit_json(&self.items, &mut buf).unwrap();
        String::from_utf8(buf).unwrap()
    }

    pub fn dominators(&mut self, options: &opt::Dominators) -> String {
        let dominators = analyze::dominators(&mut self.items, options).unwrap();
        let mut buf = Vec::new();
        dominators.emit_json(&self.items, &mut buf).unwrap();
        String::from_utf8(buf).unwrap()
    }

    pub fn paths(&mut self, options: &opt::Paths) -> String {
        let paths = analyze::paths(&mut self.items, options).unwrap();
        let mut buf = Vec::new();
        paths.emit_json(&self.items, &mut buf).unwrap();
        String::from_utf8(buf).unwrap()
    }
}
