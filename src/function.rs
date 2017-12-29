use ::Common;
use fuzzy_pickles::{Argument, Control, FunctionHeader, Visit, Visitor};
use pattern::Pattern;

pub fn args_expl(header: &FunctionHeader, common: &Common) -> Vec<String> {
    let mut args = ArgsExpl {
        common: common,
        args: vec![],
    };
    header.visit(&mut args);
    args.args
}

struct ArgsExpl<'ast> {
    common: &'ast Common<'ast>,
    args: Vec<String>,
}

impl<'src> Visitor for ArgsExpl<'src> {
    fn visit_argument(&mut self, arg: &Argument) -> Control {
        let mut buffer = String::new();
        let mut expl = Pattern { common: self.common.with(&mut buffer) };
        arg.visit(&mut expl);
        self.args.push(buffer);
        Control::Break
    }
}