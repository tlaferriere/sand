use std::collections::HashSet;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};
use syn::visit::{self, Visit};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{ItemFn, Receiver, ReturnType, Token, Type, TypeTuple, UnOp};

struct Args {
    vars: HashSet<Ident>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            vars: vars.into_iter().collect(),
        })
    }
}

/// AST visitor that identifies and causes compile errors on non synthesizable syntax.
#[derive(Default)]
struct EntityVisitor {
    errors: TokenStream,
    type_assertions: TokenStream,
}

impl<'ast> Visit<'ast> for EntityVisitor {
    /// Assert that self is copy
    fn visit_receiver(&mut self, rec: &'ast Receiver) {
        self.type_assertions.extend(
            quote_spanned! {rec.span()=> Self: std::marker::Copy,}
        )
    }

    /// Check that every function has a return type
    ///
    /// TODO: Allow type inference for closures.
    fn visit_return_type(&mut self, ret_ty: &'ast ReturnType) {
        match ret_ty {
            ReturnType::Default => {
                self.errors.extend(quote_spanned! {
                ret_ty.span()=>
                compile_error!("Cannot synthesize function without return value.");
            });
            }
            ReturnType::Type(_, ty) => {
                match &**ty {
                    Type::Tuple(
                        TypeTuple { paren_token: _, elems: _elems }) if Punctuated::is_empty(_elems) => {
                        self.errors.extend(quote_spanned! {
                    ret_ty.span()=>
                    compile_error!("Cannot synthesize function with empty return value.");
                });
                    }
                    _ => {}
                }
            }
        }
        visit::visit_return_type(self, ret_ty);
    }

    /// Find all types used to assert Copy on them
    fn visit_type(&mut self, ty: &'ast Type) {
        self.type_assertions.extend(quote_spanned! {ty.span()=> #ty: std::marker::Copy,});
        visit::visit_type(self, ty);
    }

    /// Make dereferencing illegal
    fn visit_un_op(&mut self, node: &'ast UnOp) {
        if let UnOp::Deref(_) = node {
            self.errors.extend(quote_spanned! {
                node.span()=>
                compile_error!("Explicitly dereferencing pointers is not synthesizable, please refactor your algorithm semantics to work with values.");
            });
        }
        visit::visit_un_op(self, node);
    }
}

pub fn entity(args: TokenStream, input: TokenStream) -> TokenStream {
    let entity_fn: ItemFn = syn::parse2(input).unwrap();

    let mut visitor = EntityVisitor::default();
    visitor.visit_item_fn(&entity_fn);

    let EntityVisitor { errors, type_assertions } = visitor;

    let ItemFn { attrs, vis, sig, block } = entity_fn;

    quote! {
        #vis #sig {
            {
                #errors
                struct _AssertCopy where #type_assertions;
            }
            #block
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_test() {
        // use std::{fs::File, io::Write};
        let generated = entity(TokenStream::default(), quote! {
            fn adder(a: i32, b: i32) -> i32 {
                a + b
            }
        }.into());
        assert_eq!(generated.to_string(), quote!(
            fn adder(a: i32, b: i32) -> i32 {
            {
                struct _AssertCopy
                    where i32: std::marker::Copy, i32: std::marker::Copy, i32: std::marker::Copy,;
            }
                { a + b }
            }
        ).to_string());
        // let mut file = File::create("test_entity.rs").unwrap();
        // file.write_all(format!("{}", generated).as_bytes()).unwrap();
    }

    #[test]
    fn entity_should_assert_all_types_are_copy() {
        use std::{fs::File, io::Write};
        let generated = entity(TokenStream::default(), quote! {
            fn const_adder(a: i32) -> i32 {
                let my_const: u8 = 42;
                a + my_const
            }
        }.into());
        assert_eq!(generated.to_string(), quote!(
            fn const_adder(a: i32) -> i32 {
                {
                    struct _AssertCopy
                        where i32: std::marker::Copy, i32: std::marker::Copy, u8: std::marker::Copy,;
                }
                {
                    let my_const: u8 = 42;
                    a + my_const
                }
            }
        ).to_string());
        // let mut file = File::create("test_entity.rs").unwrap();
        // file.write_all(format!("{}", generated).as_bytes()).unwrap();
    }

    #[test]
    fn entity_should_error_for_default_empty_return_type() {
        use std::{fs::File, io::Write};
        let generated = entity(TokenStream::default(), quote! {
            fn const_adder(a: i32) {
                let my_const: u8 = 42;
                a + my_const
            }
        }.into());
        assert_eq!(generated.to_string(), quote!(
            fn const_adder(a: i32) {
                {
                    compile_error!("Cannot synthesize function without return value.");
                    struct _AssertCopy
                        where i32: std::marker::Copy, u8: std::marker::Copy,;
                }
                {
                    let my_const: u8 = 42;
                    a + my_const
                }
            }
        ).to_string());
        // let mut file = File::create("test_entity.rs").unwrap();
        // file.write_all(format!("{}", generated).as_bytes()).unwrap();
    }

    #[test]
    fn entity_should_error_for_empty_return_type() {
        use std::{fs::File, io::Write};
        let generated = entity(TokenStream::default(), quote! {
            fn const_adder(a: i32) -> () {
                let my_const: u8 = 42;
                a + my_const
            }
        }.into());
        assert_eq!(generated.to_string(), quote!(
            fn const_adder(a: i32) -> () {
                {
                    compile_error!("Cannot synthesize function with empty return value.");
                    struct _AssertCopy
                        where i32: std::marker::Copy, (): std::marker::Copy, u8: std::marker::Copy,;
                }
                {
                    let my_const: u8 = 42;
                    a + my_const
                }
            }
        ).to_string());
        // let mut file = File::create("test_entity.rs").unwrap();
        // file.write_all(format!("{}", generated).as_bytes()).unwrap();
    }

    #[test]
    fn entity_should_assert_self_is_copy() {
        use std::{fs::File, io::Write};

        struct Counter {
            c: u16,
            step: u8,
        }

        let generated = entity(TokenStream::default(), quote! {
            fn increment(&self) -> Self {
                self.c + self.step
            }
        }.into());
        assert_eq!(generated.to_string(), quote!(
            fn increment(&self) -> Self {
                {
                    struct _AssertCopy
                        where Self: std::marker::Copy, Self: std::marker::Copy,;
                }
                {
                    self.c + self.step
                }
            }
        ).to_string());
        // let mut file = File::create("test_entity.rs").unwrap();
        // file.write_all(format!("{}", generated).as_bytes()).unwrap();
    }
}
