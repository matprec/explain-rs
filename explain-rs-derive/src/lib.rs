extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Explain)]
pub fn explain_derive(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).expect("Unable to parse input");

    let gen = impl_explain(&ast);

    gen.parse().expect("Unable to generate")
}


fn impl_explain(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;

    quote! {
        impl Explain for #name {
            fn paragraphs(&mut self) -> &mut Vec<Paragraph> {
                &mut self.p
            }
        }
    }
}