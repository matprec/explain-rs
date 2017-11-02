extern crate explain_rs;
extern crate fuzzy_pickles;

use std::collections::HashMap;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;

use fuzzy_pickles::Visit;

use explain_rs::*;

// This isn't meant to be pretty, but to work.
// If you disagree, feel free to open a PR!

// Ugly, but: This makes travis run this test without a shell script \( ^-^)/
#[cfg(test)]
mod test {
    #[test]
    fn main() {
        super::main()
    }
}

fn main() {
    let mut args = env::args();
    args.next();
    let local = if let Some(arg) = args.next() {
        if arg == "--local" { true } else { false }
    } else { false };

    let mut html = HTMLOutput::new(local);
    html.load("src/snippets/stable/function_args.rs");
    html.finalize();
}

struct HTMLOutput {
    local: bool,
    files: Vec<String>,
}
use std::path::Path;
use std::fs::{File, DirBuilder};
use std::io::Read;

impl HTMLOutput {
    fn new(local: bool ) -> HTMLOutput {
        HTMLOutput {
            local,
            files: vec![]
        }
    }

    fn finalize(self) {
        let mut out = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("dist/index.html")
            .unwrap();
        for file in self.files {
            writeln!(out, "<a href='{}.html'>{}</a>", &file, &file).unwrap();
        }
    }

    fn load<P: AsRef<Path>>(&mut self, path: P) {
        let mut out = {
            let path = path.as_ref().to_owned();
            let ending = path.iter().rev().next().expect("Path should exist").to_str().unwrap();
            self.files.push(ending.to_owned());
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&format!("dist/{}.html", ending))
                .unwrap()
        };
        let mut file = File::open(path).expect("File should exist");
        let mut buffer = String::new();
        file.read_to_string(&mut buffer);
        self.write(&buffer, &mut out);
    }

    fn write(&mut self, src: &str, file: &mut File) {
        let parsed = fuzzy_pickles::parse_rust_file(src);

        match parsed {
            Result::Ok(parsed) => {
                let mut context = Context {
                    buffer: String::new(),
                    source: src.to_owned(),
                    ids: vec![],
                    is_list_item: false,
                };
                {
                    let mut expl = html::Crate { ctx: &mut context };
                    parsed.visit(&mut expl);
                }
                let curr_dir = std::env::current_dir().expect("Working dir should exist");
                let (local, curr_dir) = if self.local {
                    ("file:///", curr_dir.to_str().expect("Should be unicode"))
                } else {
                    ("", "")
                };
                writeln!(
                    file,
                    r#"<head>
    <base href="{}{}">
    <link rel="stylesheet" href="explain-rs/dist/res/highlightjs/styles/default.css">
    <script src="explain-rs/dist/res/highlightjs/highlight.pack.js"></script>
    <script>hljs.initHighlightingOnLoad();</script>
                        "#,
                    local,
                    curr_dir
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
}