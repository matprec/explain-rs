use ::{Common, Convenience};
use fuzzy_pickles::{Control, PatternIdent, Visit, Visitor};

struct Idents<'ast> {
    common: &'ast Common<'ast>,
    idents: Vec<String>,
}

pub fn collect_idents<T: Visit>(t: &T, common: &Common) -> Vec<String> {
    let mut collector = Idents {
            common,
            idents: vec![]
    };
    t.visit(&mut collector);
    collector.idents
}

impl<'ast> Visitor for Idents<'ast> {
    fn visit_pattern_ident(&mut self, ident: &PatternIdent) -> Control {
        let mut buffer = String::new();
        let mut common = self.common.with(&mut buffer);
        common.push_src(ident);
        self.idents.push(buffer);
        Control::Break
    }
}