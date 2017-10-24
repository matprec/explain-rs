use fuzzy_pickles::{Argument, Extent, Function, HasExtent, PatternBox, Visit, Visitor};

use Context;

#[derive(DerefForContext)]
pub struct Crate {
    pub ctx: Context
}

impl Crate {
    pub fn new(context: Context) -> Crate {
        Crate {
            ctx: context
        }
    }
}

impl Visitor for Crate {
    fn visit_function(&mut self, function: &Function) {
        self.push_text("The function ", &function.extent());
        self.push_src_ref(&function.header.name.extent);
        self.push_text(":", &function.extent());
        self.push_newline();
    }
}