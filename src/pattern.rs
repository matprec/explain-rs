use ::{util, Common, Convenience};
use fuzzy_pickles::{Control, PatternIdent, PatternStruct, Visitor};

pub struct Pattern<'ast> {
    pub common: Common<'ast>,
}

impl<'ast> Visitor for Pattern<'ast> {
    fn visit_pattern_ident(&mut self, ident: &PatternIdent) -> Control {
        self.common.push_str("named ");
        self.common.push_src(ident);
        Control::Break
    }

    fn visit_pattern_struct(&mut self, pattern: &PatternStruct) -> Control {
        let idents = util::collect_idents(pattern, &self.common);
        let idents: String = idents.join(", ");
        self.common.push_str(&format!("destructured into the fields {}", idents));
        Control::Break
    }
}