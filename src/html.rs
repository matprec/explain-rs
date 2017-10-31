use fuzzy_pickles::{Argument, Control, Extent, Function, HasExtent, PatternBox, PatternIdent, PatternName, PatternStruct, Visit, Visitor};

use Context;
use std::ops::{Deref, DerefMut};

pub struct Crate<'ctx> {
    pub ctx: &'ctx mut Context
}

impl<'ctx> Visitor for Crate<'ctx> {
    fn visit_function(&mut self, function: &Function) -> Control {
        self.ctx.push_text("The function ", &function.extent());
        self.ctx.push_src_ref(&function.header.name.extent);
        self.ctx.push_text(":", &function.extent());

        {
            let mut argument = FnArgument { ctx: self.ctx, count: 0 };
            function.header.visit(&mut argument)
        }

        self.ctx.push_newline();
        Control::Continue
    }
}

pub struct FnArgument<'ctx> {
    ctx: &'ctx mut Context,
    count: usize
}
impl<'ctx> Visitor for FnArgument<'ctx> {
    fn visit_argument(&mut self, arg: &Argument) -> Control {
        self.count += 1;
        println!("{:?}", arg);
        Control::Continue
    }

    fn visit_pattern_ident(&mut self, ident: &PatternIdent) -> Control {
        self.ctx.push_list_item();
        self.ctx.push_text("takes an argument called ", &ident.extent());
        self.ctx.push_src_ref(&ident.extent());
        Control::Break
    }

    fn visit_pattern_name(&mut self, pattern: &PatternName) -> Control {
        self.ctx.push_text("NÖP", &pattern.extent());
        Control::Break
    }

    fn visit_pattern_struct(&mut self, pattern: &PatternStruct) -> Control {
        let pos = self.pos();
        self.ctx.push_list_item();
        let idents = Identifier::strip_from(pattern);
        let text = format!("destructures its {} argument into its field{} ",
                           pos,
                           if idents.len() != 1 { "s" } else { "" });
        self.ctx.push_text(&text, &pattern.extent());
        let mut first = true;
        for ident in idents {
            if !first {
                self.ctx.push_text(", ", &pattern.extent());
            } else {
                first = false;
            }
            self.ctx.push_src_ref(&ident);
        }
        Control::Break
    }
}

impl<'ctx> FnArgument<'ctx> {
    fn pos(&self) -> String {
        match self.count {
            0 => unreachable!(),
            1 => "first".to_owned(),
            2 => "second".to_owned(),
            3 => "third".to_owned(),
            _ => format!("{}nd", self.count)
        }
    }
}

#[derive(Default)]
struct Identifier {
    idents: Vec<Extent>
}

impl Visitor for Identifier {
    fn visit_pattern_ident(&mut self, ident: &PatternIdent) -> Control {
        self.idents.push(ident.extent().clone());
        Control::Continue
    }
}

impl Identifier {
    fn strip_from<T>(t: &T) -> Vec<Extent> where T: Visit {
        let mut ident = Identifier::default();
        t.visit(&mut ident);
        ident.idents
    }
}
