#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(conservative_impl_trait)]
#![feature(try_from)]
#![feature(nll)]

extern crate fuzzy_pickles;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod function;
mod linkify;
mod pattern;
mod types;
mod util;

#[allow(dead_code)]
#[allow(unused_variables)]
mod snippets;

use fuzzy_pickles::{Control, FunctionHeader, HasExtent, Item, Visit, Visitor};
use std::convert::TryFrom;
use std::mem;

pub enum Mode {
    Syntax,
    Controlflow,
}

impl TryFrom<String> for Mode {
    type Error = ();

    fn try_from(mode: String) -> Result<Mode, ()> {
        let mode: &str = &mode;
        match mode {
            "syntax" => Ok(Mode::Syntax),
            "cf" => Ok(Mode::Controlflow),
            _ => Err(())
        }
    }
}

pub struct Common<'ast> {
    cursor: usize,
    src: &'ast str,
    buffer: &'ast mut String,
}

impl<'ast> Common<'ast> {
    fn with(&self, buffer: &'ast mut String) -> Common<'ast> {
        Common {
            cursor: self.cursor,
            src: &self.src,
            buffer: buffer,
        }
    }
}

impl<'ast, T: Convenience> From<&'ast mut T> for Common<'ast> {
    fn from(t: &'ast mut T) -> Common<'ast> {
        let Common { cursor, src, buffer } = t.borrow();
        Common {
            cursor,
            src,
            buffer,
        }
    } 
}

impl<'ast> Convenience for Common<'ast> {
    fn borrow<'bst>(&'bst mut self) -> Common<'bst> {
        Common {
            cursor: self.cursor,
            src: &self.src,
            buffer: &mut self.buffer,
        }
    }
}

struct SyntaxExpl<'ast> {
    common: Common<'ast>,
}

impl<'ast> Convenience for SyntaxExpl<'ast> {
    fn borrow<'bst>(&'bst mut self) -> Common<'bst> {
        From::from(&mut self.common)
    }
}

impl<'ast> Visitor for SyntaxExpl<'ast> {
    fn visit_item(&mut self, item: &Item) -> Control {
        if item.contains(self.common.cursor) {
            Control::Continue
        } else {
            Control::Break
        }
    }

    fn visit_function_header(&mut self, header: &FunctionHeader) -> Control {
        self.push_str("The function ");
        let ident = header.name.clone();
        self.push_src(&ident);
        self.push_str(" takes ");

        let mut args = function::args_expl(header, &self.common);
        args.iter_mut().for_each(|arg| {
            let new_arg = format!("is {}", arg);
            mem::replace(arg, new_arg);
        });
        
        match args.len() {
            0 => self.push_str("no arguments."),
            _ => {
                let s = if args.len() == 1 {format!(", which {}.",  args[0])} else {"s:".to_string()};
                self.push_str(&format!("{} argument{}", args.len(),  s));
                if args.len() != 1 {
                    args.last_mut().map(|expl| expl.push('.'));
                    self.push_list(&args);
                }
            },
        }
        Control::Continue
    }
}


pub fn syntax(src: String, cursor: usize) -> Result<String, String> {
    let file = fuzzy_pickles::parse_rust_file(&src).map_err(|_| "Couldn't parse snippet".to_string())?;
    let mut buffer = String::new();
    let common = Common { cursor, src: &src, buffer: &mut buffer };
    let mut syntax = SyntaxExpl { common };
    file.visit(&mut syntax);
    linkify::linkify(&mut buffer);
    Ok(buffer)
}

trait Contains {
    fn contains(&self, cursor: usize) -> bool;
}

impl<T: HasExtent> Contains for T {
    fn contains(&self, cursor: usize) -> bool {
        let (lower,  upper) = self.extent();
        lower <= cursor && cursor <= upper
    }
}

pub trait Convenience {
    fn borrow<'ast>(&'ast mut self) -> Common;

    fn push_str(&mut self, text: &str) {
        let Common { buffer, ..} = self.borrow();
        buffer.push_str(text);
    }

    fn get_src<T>(&mut self, t: &T) -> &str where T: HasExtent {
        let Common { src, ..} = self.borrow();
        let (lower, upper) = t.extent();
        &src[lower..upper]
    }

    fn push_src<T>(&mut self, t: &T) where T: HasExtent {
        let Common { cursor, src, buffer } = self.borrow();
        buffer.push_str("<code>");
        if t.contains(cursor) {
            buffer.push_str("<strong>");
        }
        let (lower, upper) = t.extent();
        buffer.push_str(&src[lower..upper]);
        if t.contains(cursor) {
            buffer.push_str("</strong>");
        }
        buffer.push_str("</code>");
    }

    fn push_list(&mut self, texts: &[String]) {
        let Common { buffer, ..} = self.borrow();
        buffer.push_str("<ol>");
        for text in texts {
            buffer.push_str("<li>");
            buffer.push_str(&text);
            buffer.push_str("</li>");
        }
        buffer.push_str("</ol>");
    }

}