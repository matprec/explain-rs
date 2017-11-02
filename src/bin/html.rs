extern crate explain_rs;
extern crate fuzzy_pickles;

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

use fuzzy_pickles::Visit;

use explain_rs::*;

fn main() {
    let src = include_str!("../snippets/stable/function_args.rs");
    let file = fuzzy_pickles::parse_rust_file(src);

    match file {
        Result::Ok(file) => {
            let mut context = Context {
                buffer: String::new(),
                source: src.to_owned(),
                ids: vec![],
                is_list_item: false,
            };
            {
                let mut expl = html::Crate { ctx: &mut context };
                file.visit(&mut expl);
            }
            let curr_dir = std::env::current_dir().expect("Working dir should exist");
            let mut file = OpenOptions::new()
                .write(true)
                .read(true)
                .create(true)
                .truncate(true)
                .open("target/index.html")
                .unwrap();
            writeln!(
                file,
                r#"<head>
<base href="file:///{}">
<link rel="stylesheet" href="explain-rs/res/highlightjs/styles/default.css">
<script src="explain-rs/res/highlightjs/highlight.pack.js"></script>
<script>hljs.initHighlightingOnLoad();</script>
                    "#,
                curr_dir.display()
            ).unwrap();
            writeln!(
                file,
                "
                <style>
                .pre {{
                    font-family: monospace;
                    background-color: lightgrey;
                }}
                code {{
                    white-space: pre-wrap;
                }}
                {} {{\
                    background-color: yellow;\
                }}</style></head>",
                context.css()
            ).unwrap();
            writeln!(file, "{}", context.buffer).unwrap();
            writeln!(file, "<pre><code>{}</code></pre>", &context.src()).unwrap();
        }
        Result::Err(err) => {
            let detail = err.with_text(&src);
            println!("{}", detail)
        }
    }
}