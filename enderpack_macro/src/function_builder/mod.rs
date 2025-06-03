use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::ops::Deref;
use syn::*;

pub fn wrap_statement(statement: Stmt) -> syn::Result<TokenStream2> {
    match statement {
        Stmt::Expr(expr, _semi) => Ok(quote! { add_command(#expr) }),
        Stmt::Local(local) => {
            dbg!(&local.pat);
            let name = crate::helpers::get_variable_name(&local.pat)?;
            let init = &local.init;
            match init {
                Some(local_init) => handle_let(local_init, &name),
                None => Err(Error::new_spanned(
                    &local,
                    "Let binding needs to be initalized",
                )),
            }
        }
        _ => Err(Error::new_spanned(statement, "Unsupported item")),
    }
}

fn handle_let(local_init: &LocalInit, name: &str) -> syn::Result<TokenStream2> {
    match local_init.expr.deref() {
        // Unsigned
        Expr::Lit(expr_lit) => parse_litteral(expr_lit, name, false),
        // Signed
        Expr::Unary(expr_unary) => parse_unary(expr_unary, name),
        _ => Err(Error::new_spanned(
            local_init.expr.deref(),
            "Unsupported let binding",
        )),
    }
}

fn parse_unary(expr_unary: &ExprUnary, name: &str) -> syn::Result<TokenStream2> {
    match expr_unary.op {
        UnOp::Neg(_) => match expr_unary.expr.deref() {
            Expr::Lit(expr_lit) => parse_litteral(expr_lit, name, true),
            Expr::Unary(expr_unary) => parse_unary(expr_unary, name),
            _ => Err(Error::new_spanned(
                expr_unary.expr.deref(),
                "Unsupported expression in unary",
            )),
        },
        _ => Err(Error::new_spanned(expr_unary, "Unsupported unary operator")),
    }
}

fn parse_litteral(expr_lit: &ExprLit, name: &str, negative: bool) -> syn::Result<TokenStream2> {
    match &expr_lit.lit {
        Lit::Int(int) => {
            if negative {
                Ok(quote! { add_scoreboard(#name, Some(-#int)) })
            } else {
                Ok(quote! { add_scoreboard(#name, Some(#int)) })
            }
        }
        _ => Err(Error::new_spanned(expr_lit, "Unsupported litteral")),
    }
}
