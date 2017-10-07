#[allow(dead_code)]
mod impl_trait;

#[cfg(test)]
mod tests {
    //use snippets::explain;

    #[test]
    fn impl_trait() {
        let source = include_str!("impl_trait.rs");
        //explain(source)
    }
}