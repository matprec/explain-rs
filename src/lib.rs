
#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(conservative_impl_trait)]

#[macro_use]
extern crate explain_rs_derive;

extern crate fuzzy_pickles;

use fuzzy_pickles::Extent;

mod snippets;
pub mod html;

use std::cmp;
use std::collections::HashMap;
use std::fmt::Write;

type Extented = Extent;

trait ContainsExt {
    fn contains(&self, ext: Extented) -> bool;
}

impl ContainsExt for Extented {
    fn contains(&self, ext: Extented) -> bool {
        self.0 <= ext.0 && ext.1 <= self.1
    }
}

#[derive(Debug)]
pub struct ExtentOrNested {
    extent: Extented,
    subs: Vec<ExtentOrNested>,
}

impl ExtentOrNested {
    fn new(extent: Extented) -> ExtentOrNested {
        ExtentOrNested {
            extent,
            subs: vec![],
        }
    }

    fn contains(&self, extented: Extented) -> bool {
        self.extent.contains(extented)
    }

    fn push(&mut self, extent: Extented) -> Result<String, Extented> {
        if !self.contains(extent) {
            Err(extent)
        } else {
            Ok(self.subs.push_ext(extent))
        }
    }
}

trait ExtentExt {
    fn push_ext(&mut self, extent: Extented) -> String;

    fn src(&self, buffer: &mut String, source: &str, init: usize) -> usize;

    fn css(&self, stack: &mut Vec<usize>, selectors: &mut Vec<String>);
}

impl ExtentExt for Vec<ExtentOrNested> {
    fn push_ext(&mut self, extent: Extented) -> String {
        let result = self.iter_mut()
            .enumerate()
            .find(|&(_, ref it)| it.contains(extent))
            .map(|(index, it)| if it.extent == extent {
                format!("_{}", index)
            } else {
                let sub_id = it.push(extent).expect("Bug in sub elements!");
                format!("_{}{}", index, sub_id)
            });

        // TODO: Ugh fixme NLL
        if let Some(id) = result {
            id
        } else {
            let id = self.len();
            self.push(ExtentOrNested::new(extent));
            format!("_{}", id)
        }
    }

    fn src(&self, buffer: &mut String, source: &str, init: usize) -> usize {
        self.iter().enumerate().fold(init, |last, (index, ext)| {
            if last < ext.extent.0 {
                buffer.push_str(&source[last..ext.extent.0].replace('<', "&lt").replace(
                    '>',
                    "&gt",
                ));
            }
            buffer.push_str(&format!("<span class='c{}'>", index));
            let last = ext.subs.src(buffer, source, ext.extent.0);
            let snippet = &source[cmp::max(ext.extent.0, last)..ext.extent.1]
                .replace('<', "&lt")
                .replace('>', "&gt");
            buffer.push_str(snippet);
            buffer.push_str("</span>");
            ext.extent.1
        })
    }

    fn css(&self, stack: &mut Vec<usize>, selectors: &mut Vec<String>) {
        self.iter().enumerate().fold(
            stack,
            |mut stack, (index, ext)| {
                stack.push(index);

                let ids: Vec<String> = stack.iter().map(|i| i.to_string()).collect();
                let ids = ids.join("_");

                let ids2: Vec<String> = stack.iter().map(|i| i.to_string()).collect();
                let ids2 = ids2.join(" > .c");

                let id_string = format!("expl_{}", ids);
                let selector = format!("#{}:hover ~ pre > code > .c{}", id_string, ids2);
                selectors.push(selector);

                ext.subs.css(stack, selectors);

                let value = stack.pop().expect("BUG: Index already got removed");
                assert!(value == index);
                stack
            },
        );
    }
}

pub struct Context {
    pub buffer: String,
    pub source: String,
    pub ids: Vec<ExtentOrNested>,
    pub is_list_item: bool,
}

impl Context {
    pub fn push_text(&mut self, text: &str, extent: &Extent) {
        let id = self.ids.push_ext(extent.clone());
        write!(self.buffer, "<span id='expl{}'>{}</span>", id, text).unwrap()
    }

    pub fn push_src_ref(&mut self, extent: &Extent) {
        let id = self.ids.push_ext(extent.clone());
        let source = &self.source[extent.0..extent.1];
        write!(
            self.buffer,
            "<span class='pre' id='expl{}' >{}</span>",
            id,
            source
        ).unwrap()
    }

    pub fn push_link(&mut self, text: &str, link: &str, extent: &Extent) {
        let id = self.ids.push_ext(extent.clone());
        write!(self.buffer, "<a id='expl{}' href='{}'>{}</a>", id, link, text).unwrap()
    }

    pub fn push_list_item(&mut self) {
        self.push_newline();
        self.buffer.push_str(" - ");
    }

    pub fn push_newline(&mut self) {
        self.buffer.push_str("<br/>")
    }

    pub fn src(&self) -> String {
        let mut buffer = String::new();
        let last = self.ids.src(&mut buffer, &self.source, 0);
        buffer.push_str(&self.source[last..]);
        buffer
    }

    pub fn css(&self) -> String {
        let mut stack = vec![];
        let mut selectors = vec![];
        self.ids.css(&mut stack, &mut selectors);
        selectors.join(", ")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_extent_cache() {
        let mut extents: Vec<ExtentOrNested> = vec![];
        assert!(extents.push_ext((0, 5)) == "_0");
        assert!(extents.push_ext((1, 2)) == "_0_0");
        assert!(extents.push_ext((3, 4)) == "_0_1");
        assert!(extents.push_ext((3, 4)) == "_0_1");
    }
}
