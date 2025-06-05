mod let_handler;

use let_handler::FunctionLet;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::*;

use crate::LetHandler;

pub fn wrap_statement(statement: Stmt) -> syn::Result<Vec<TokenStream2>> {
    match statement {
        Stmt::Expr(expr, _semi) => Ok(vec![quote! { f.add_command(&#expr); }]),
        Stmt::Local(local) => {
            dbg!(&local.pat);
            let name = crate::helpers::get_variable_name(&local.pat)?;
            let init = &local.init;
            match init {
                Some(local_init) => FunctionLet::new().handle_local_init(local_init, &name),
                None => Err(Error::new_spanned(
                    &local,
                    "Let binding needs to be initalized",
                )),
            }
        }
        _ => Err(Error::new_spanned(statement, "Unsupported item")),
    }
}
