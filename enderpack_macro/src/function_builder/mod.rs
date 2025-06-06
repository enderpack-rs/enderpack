use std::ops::Deref;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::*;

use crate::helpers::get_variable_name;

pub fn wrap_statement(statement: Stmt) -> syn::Result<Vec<TokenStream2>> {
    match statement {
        Stmt::Expr(expr, _semi) => Ok(vec![quote! { f.add_command(&#expr); }]),
        Stmt::Local(local) => handle_let(local),
        _ => Err(Error::new_spanned(statement, "Unsupported item")),
    }
}

fn handle_let(local: Local) -> syn::Result<Vec<TokenStream2>> {
    let Pat::Type(pat_type) = &local.pat else {
        return Ok(vec![quote! { #local }]);
    };
    let name = get_variable_name(&pat_type.pat)?;
    let type_ident = match pat_type.ty.deref() {
        Type::Path(type_path) => match type_path.path.segments.last() {
            Some(path_segment) => path_segment.ident.clone(),
            None => return Err(Error::new_spanned(type_path, "Invalid type path")),
        },
        _ => return Ok(vec![quote! { #local }]),
    };
    let init_expr = match &local.init {
        Some(local_init) => local_init.expr.clone(),
        None => {
            return Err(Error::new_spanned(
                &local,
                "Variable declaration is missing initialisation",
            ));
        }
    };
    match quote! {#type_ident}.to_string().as_str() {
        "Score" => Ok(vec![
            quote! {let #name = #init_expr;},
            quote! {let #name = #type_ident::new(
                stringify!(#name),
                &format!("{}.{}", f.get_path().replace("::", "."), f.get_name()),
                #name
            );},
            quote! {f.add_variable(&#name);},
        ]),
        _ => Ok(vec![quote! { #local }]),
    }
}
