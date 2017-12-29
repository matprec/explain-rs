#![feature(try_from)]

extern crate explain_rs;
extern crate serde_json;
#[macro_use]
extern crate stdweb;

use explain_rs::{ Mode };
use std::convert::TryFrom;

pub enum ParseResult {}

fn explain_js(src: String, mode: String, cursor: usize) -> String {
    let expl = explain(src, mode, cursor);
    serde_json::to_string(&expl).unwrap()
}

fn explain(src: String, mode: String, cursor: usize) -> Result<String, String> {
    let mode = Mode::try_from(mode).map_err(|_| "Unknown Mode!".to_string())?;
    match mode {
        Mode::Syntax => explain_rs::syntax(src, cursor),
        _ => unimplemented!(),
    }
}

fn main() {
    stdweb::initialize();

    js! {
        Module.exports.explain = @{explain_js};
    }
}