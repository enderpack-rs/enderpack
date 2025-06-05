mod function_builder;
mod helpers;

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
    let mut function_stmts: Vec<Stmt> = Vec::new();
    for stmt in stmts {
        match function_builder::wrap_statement(stmt) {
            Ok(s) => s
                .iter()
                .for_each(|wrapped_stmt| function_stmts.push(parse_quote!(#wrapped_stmt))),
            Err(e) => return TokenStream::from(e.to_compile_error()),
        }
    }
    let make_func = quote! {
        #vis #sig -> Function {
            let mut f = Function::new(stringify!(#name), module_path!());
            #(#function_stmts)*
            f
        }
    };
    make_func.into()
}

trait LetHandler {
    fn handle_local_init(
        &self,
        local_init: &LocalInit,
        name: &Ident,
    ) -> syn::Result<Vec<TokenStream2>>;
    fn parse_unary(&self, expr_unary: &ExprUnary, name: &Ident) -> syn::Result<Vec<TokenStream2>>;
    fn parse_litteral(
        &self,
        expr_lit: &ExprLit,
        name: &Ident,
        negative: bool,
    ) -> syn::Result<Vec<TokenStream2>>;
}
