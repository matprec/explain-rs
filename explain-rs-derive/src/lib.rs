extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(DerefForContext)]
pub fn deref_derive(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).expect("Unable to parse input");

    let gen = impl_deref_for_context(&ast);

    gen.parse().expect("Unable to generate")
}

fn impl_deref_for_context(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;

    quote! {
        use std::ops::{Deref, DerefMut};

        impl Deref for #name {
            type Target = Context;

            fn deref(&self) -> &Context {
                &self.ctx
            }
        }

        impl DerefMut for #name {
            fn deref_mut(&mut self) -> &mut Context {
                &mut self.ctx
            }
        }
    }
}