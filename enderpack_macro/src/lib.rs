mod function_builder;
mod helpers;

use proc_macro::TokenStream;
use quote::quote;
use syn::*;

#[proc_macro_attribute]
pub fn func(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let name = &sig.ident;
    let stmts = input_fn.block.stmts;
    let let_stmts: Vec<Stmt> = stmts
        .clone()
        .into_iter()
        .filter(|stmt| matches!(stmt, Stmt::Local(_)))
        .collect();
    let mut function_stmts: Vec<Expr> = Vec::new();
    for stmt in stmts {
        match function_builder::wrap_statement(stmt) {
            Ok(s) => function_stmts.push(parse_quote!(#s)),
            Err(e) => return TokenStream::from(e.to_compile_error()),
        }
    }
    let make_func = quote! {
        #vis #sig -> Function {
            #(#let_stmts)*
            Function::new(stringify!(#name))
                .set_path(module_path!())
                #(.#function_stmts)*
        }
    };
    make_func.into()
}
