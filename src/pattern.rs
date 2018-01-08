use ::{util, Common, Convenience};
use fuzzy_pickles::{Control, PatternIdent, PatternTuple, PatternStruct, Visitor};

pub struct Pattern<'ast> {
    pub common: Common<'ast>,
}

impl<'ast> Visitor for Pattern<'ast> {
    fn visit_pattern_ident(&mut self, ident: &PatternIdent) -> Control {
        if self.common.get_src(ident) != "_" {
            self.common.push_str("named ");
            self.common.push_src(ident);
        } else {
            self.common.push_str("ignored")
        }
        Control::Break
    }

    fn visit_pattern_struct(&mut self, pattern: &PatternStruct) -> Control {
        let idents = util::collect_idents(pattern, &self.common);
        let idents: String = idents.join(", ");
        self.common.push_str(&format!("destructured into the fields {}", idents));
        Control::Break
    }

    fn visit_pattern_tuple(&mut self, pattern: &PatternTuple) -> Control {
        let idents = util::collect_idents(pattern, &self.common);
        let idents: String = idents.join(", ");
        self.common.push_str(&format!("destructured into the tuple fields {}", idents));
        Control::Break
    }
}