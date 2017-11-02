//use explain::Explanation;

#[allow(dead_code)]
#[allow(unused_variables)]
mod nightly;
#[allow(dead_code)]
#[allow(unused_variables)]
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
