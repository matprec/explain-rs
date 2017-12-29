use std::mem;

pub fn linkify(buf: &mut String) {
    let mut buffer = buf.replace("function",  "<a href='https://doc.rust-lang.org/book/second-edition/ch03-03-how-functions-work.html'>function</a>");
    buffer = buffer.replace("arguments", "<a href='https://doc.rust-lang.org/book/second-edition/ch03-03-how-functions-work.html#function-parameters'>arguments</a>");
    mem::replace(buf, buffer);
}