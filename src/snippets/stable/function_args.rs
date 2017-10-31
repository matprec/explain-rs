use snippets::{Foo, Bar, Baz};

pub fn identifier(foo: Foo) {}

pub fn destructure(Foo{x, y, z}: Foo) {}

pub fn destructure_tuplestruct(Bar::Baz(x): Bar) {}

pub fn destructure_tuple((x, y): Baz) {}

pub fn ignore(_: Foo) {}

pub fn unbox(box f: Box<Foo>) {}