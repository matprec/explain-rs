#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(conservative_impl_trait)]

#[macro_use]
extern crate explain_rs_derive;

extern crate fuzzy_pickles;

use fuzzy_pickles::{Argument, Extent, Function, HasExtent, PatternBox, Visit, Visitor};

mod snippets;

enum Aspect {
    Text(String),
    Link{title: String, href: String},
    SourceRef(Extent),
}

struct Paragraph {
    leading: (Extent, Vec<Aspect>),
    aspects: Vec<(Extent, Vec<Aspect>)>
}

fn printHTML(expl: &Explanation) {
    use std::fs::OpenOptions;
    use std::io::prelude::*;

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("target/index.html").unwrap();
    write!(file, "Hello World!").unwrap();
}

#[derive(Default, Explain)]
pub struct Explanation {
    p: Vec<Paragraph>
}

#[derive(Default, Explain)]
struct FunctionHeaderExplanation {
    p: Vec<Paragraph>,
    args: u32,
}


macro_rules! aspects {
    ( $( $variant:tt: $expression:expr),* ) => {
        vec![
            $( aspect!($variant: $expression) ),*
        ]
    };
}

macro_rules! aspect {
    ( text: $text:expr  ) => { Aspect::Text( $text.to_owned() )};
    ( src: $src:expr ) => { Aspect::SourceRef( $src ) };
    ( link: $title:expr => $href:expr ) => { Aspect::Link {
        title: $title.to_owned(),
        href: $href.to_owned()
    } };
}

impl Visitor for Explanation {

    fn visit_function(&mut self, function: &Function) {
        let mut expl = FunctionHeaderExplanation::default();
        expl.paragraph(function.extent,aspects!(
            text: "The function ",
            src: function.header.name.extent,
            text: ":"
        ));
        function.visit(&mut expl);
        self.paragraphs().append(expl.paragraphs())
    }
}

impl Visitor for FunctionHeaderExplanation {
    fn visit_argument(&mut self, arg: &Argument) {
        self.args += 1;
    }

    fn visit_pattern_box(&mut self, kw_box: &PatternBox) {
        let arg_count = self.arg_count();
        self.aspect(kw_box.extent(), aspects!(
            text: "unboxes its ",
            text: arg_count,
            text: " argument"
        ))
    }
}

trait Explain {
    fn paragraphs(&mut self) -> &mut Vec<Paragraph>;

    fn paragraph(&mut self, extent: Extent, message: Vec<Aspect>) {
        let x = Paragraph {
            leading: (extent, message),
            aspects: vec![]
        };
        self.paragraphs().push(x)
    }

    fn aspect(&mut self, extent: Extent, aspects: Vec<Aspect>) {
        self.paragraphs()
            .last_mut()
            .expect("Bug: No paragraph was pushed!")
            .aspects
            .push((extent, aspects))
    }
}

impl FunctionHeaderExplanation {
    fn arg_count(&mut self) -> String {
        match self.args {
            1 => "first".to_owned(),
            2 => "second".to_owned(),
            3 => "third".to_owned(),
            _ if self.args > 3 => self.args.to_string(),
            _ => unreachable!()
        }
    }
}

#[cfg(test)]
mod test {
    use fuzzy_pickles;
    use fuzzy_pickles::{Visit};

    use Explanation;
    use printHTML;

    #[test]
    fn run_snippets() {
        let src = include_str!("snippets/stable/function_args.rs");
        let file = fuzzy_pickles::parse_rust_file(src);
        match file {
            Result::Ok(file) => {
                let mut expl = Explanation::default();
                file.visit(&mut expl);
                printHTML(&expl)
            }
            Result::Err(err) =>  {
                let detail = err.with_text(&src);
                println!("{}", detail)
            }
        }
    }
}