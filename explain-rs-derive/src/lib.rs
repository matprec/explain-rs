extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(DerefForContext)]
pub fn deref_derive(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).expect("Unable to parse input");

    let gen = impl_deref_for_context(&ast);

    gen.parse().expect("Unable to generate")
}

fn impl_deref_for_context(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause)= ast.generics.split_for_impl();
    quote! {
        impl #impl_generics Deref for #name #ty_generics #where_clause {
            type Target = Context;

            fn deref(&self) -> &Context {
                self.ctx
            }
        }

        impl #impl_generics DerefMut for #name #ty_generics #where_clause {
            fn deref_mut(&mut self) -> &mut Context {
                self.ctx
            }
        }
    }
}