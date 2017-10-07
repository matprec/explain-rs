extern crate fuzzy_pickles;
use fuzzy_pickles::{Visit};

extern crate explain_rs;
use explain_rs::Explanation;

fn main() {
    println!("Test");
    let src = include_str!("snippets/stable/function_args.rs");
    let file = fuzzy_pickles::parse_rust_file(src);
    match file {
        Result::Ok(file) => {
            let mut expl = Explanation::default();
            println!("1");
            file.visit(&mut expl)
        }
        Result::Err(err) =>  {
            let detail = err.with_text(&src);
            println!("{}", detail)
        }
    }
}