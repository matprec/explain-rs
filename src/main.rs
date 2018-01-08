#![feature(match_default_bindings)]
#![feature(nll)]
#![feature(try_from)]

             extern crate explain_rs;
             extern crate itertools;
#[macro_use] extern crate lazy_static;
             extern crate serde_json;
#[macro_use] extern crate stdweb;
             extern crate url;

use explain_rs::{ Mode };
use std::collections::HashMap;
use std::convert::TryFrom;

lazy_static! {
    static ref EXAMPLES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Function Args", include_str!("snippets/stable/function_args.rs"));
        m
    };
}

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

fn list_examples() -> String {
    let keys: Vec<String> = EXAMPLES.keys().map(|key| key.to_string()).collect();
    serde_json::to_string(&keys).unwrap()
}

fn load_example(name: String) -> String {
    let name: &str = &name;
    EXAMPLES[name].to_string()
}

fn fetch_src_js(address: String) -> String {
    serde_json::to_string(&fetch_src(address)).unwrap()
}

fn fetch_src(address: String) -> Result<(), String> {
    use itertools::Itertools;
    use url::Url;

    let mut url = Url::parse(&address).map_err(|_| "Couldn't parse url".to_string())?;
    // Rewrite url if we know how to deal with a specific host
    let url: String = if let Some(host) = url.host_str() {
        match host {
            "github.com" => {
                let new_path: String = url.path_segments()
                    .unwrap()
                    .enumerate()
                    .filter(|(index, path)| {
                        if *index == 2 && *path == "blob" {
                            false
                        } else {
                            true
                        }
                    })
                    .map(|(_, path)| path)
                    .join("/");
                url.set_path(&new_path);
                url.set_host(Some("raw.githubusercontent.com")).unwrap();
                url
            }
            _ => url
        }
    } else {
        return Err("No host in address".to_string())
    }.into_string();
    js! {
        $.ajax({
            type: "GET",
            url: @{url},
            success: function(data) {
                console.log(data);
                load_src(data);
            }
        });
    };
    Ok(())
}

fn main() {
    stdweb::initialize();

    js! {
        Module.exports.explain = @{explain_js};
        Module.exports.fetch_src = @{fetch_src_js};
        Module.exports.list_examples = @{list_examples};
        Module.exports.load_example = @{load_example};
    }
}