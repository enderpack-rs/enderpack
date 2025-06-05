use crate::LetHandler;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::ops::Deref;
use syn::*;

pub struct FunctionLet;

impl FunctionLet {
    pub fn new() -> FunctionLet {
        FunctionLet {}
    }
}

impl LetHandler for FunctionLet {
    fn handle_local_init(
        &self,
        local_init: &LocalInit,
        name: &Ident,
    ) -> syn::Result<Vec<TokenStream2>> {
        match local_init.expr.deref() {
            // Unsigned
            Expr::Lit(expr_lit) => self.parse_litteral(expr_lit, name, false),
            // Signed
            Expr::Unary(expr_unary) => self.parse_unary(expr_unary, name),
            _ => Err(Error::new_spanned(
                local_init.expr.deref(),
                "Unsupported let binding",
            )),
        }
    }

    fn parse_unary(&self, expr_unary: &ExprUnary, name: &Ident) -> syn::Result<Vec<TokenStream2>> {
        match expr_unary.op {
            UnOp::Neg(_) => match expr_unary.expr.deref() {
                Expr::Lit(expr_lit) => self.parse_litteral(expr_lit, name, true),
                Expr::Unary(expr_unary) => self.parse_unary(expr_unary, name),
                _ => Err(Error::new_spanned(
                    expr_unary.expr.deref(),
                    "Unsupported expression in unary",
                )),
            },
            _ => Err(Error::new_spanned(expr_unary, "Unsupported unary operator")),
        }
    }

    fn parse_litteral(
        &self,
        expr_lit: &ExprLit,
        name: &Ident,
        negative: bool,
    ) -> syn::Result<Vec<TokenStream2>> {
        let name_str = name.to_string();
        match &expr_lit.lit {
            Lit::Int(int) => {
                let int = if negative {
                    quote! {-#int}
                } else {
                    quote! {#int}
                };
                Ok(vec![
                    quote! {
                        let #name = enderpack::data_types::variables::score::Score::new(
                            &#name_str,
                            &format!("{}.{}", f.get_path().replace("::", "."), f.get_name()),
                            #int
                        );
                    },
                    quote! {
                        f.add_variable(#name);
                    },
                ])
            }
            _ => Err(Error::new_spanned(expr_lit, "Unsupported litteral")),
        }
    }
}
