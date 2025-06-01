use std::ops::Deref;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::*;

#[proc_macro_attribute]
pub fn func(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let name = &sig.ident;
    let stmts = input_fn.block.stmts;
    let mut wrapped_stmts: Vec<Expr> = Vec::new();
    for stmt in stmts {
        match wrap_statement(stmt) {
            Ok(s) => wrapped_stmts.push(parse_quote!(#s)),
            Err(e) => return TokenStream::from(e.to_compile_error()),
        }
    }
    let make_func = quote! {
        #vis #sig -> Function {
            Function::new(stringify!(#name))
                .set_path(module_path!())
                #(.#wrapped_stmts)*
        }
    };
    make_func.into()
}

fn wrap_statement(statement: Stmt) -> syn::Result<TokenStream2> {
    match statement {
        Stmt::Expr(expr, _semi) => Ok(quote! { add_command(#expr) }),
        Stmt::Local(local) => {
            dbg!(&local.pat);
            let name = get_name(&local.pat)?;
            let init = &local.init;
            match init {
                Some(local_init) => handle_let(local_init, name),
                None => Err(Error::new_spanned(
                    &local,
                    "Let binding needs to be initalized",
                )),
            }
        }
        _ => Err(Error::new_spanned(statement, "Unsupported item")),
    }
}

fn handle_let(local_init: &LocalInit, name: String) -> syn::Result<TokenStream2> {
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

fn parse_unary(expr_unary: &ExprUnary, name: String) -> syn::Result<TokenStream2> {
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

fn parse_litteral(expr_lit: &ExprLit, name: String, negative: bool) -> syn::Result<TokenStream2> {
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

fn get_name(pat: &Pat) -> syn::Result<String> {
    match pat {
        Pat::Ident(ident) => Ok(ident.ident.to_string()),
        Pat::Type(ty) => match ty.pat.deref() {
            Pat::Ident(ident) => Ok(ident.ident.to_string()),
            _ => Err(Error::new_spanned(ty, "Unsupported let binding")),
        },
        _ => Err(Error::new_spanned(pat, "Unsupported let binding")),
    }
}
