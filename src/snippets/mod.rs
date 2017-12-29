//use explain::Explanation;

mod nightly;
mod stable;

pub struct Foo {
    x: u32,
    y: u32,
    z: u32,
}

pub enum Bar {
    Baz(Baz),
}

type Baz = (i32, u32);
