use std::ops::Deref;
use syn::*;

pub fn get_variable_name(pat: &Pat) -> syn::Result<Ident> {
    match pat {
        Pat::Ident(ident) => Ok(ident.ident.clone()),
        Pat::Type(ty) => match ty.pat.deref() {
            Pat::Ident(ident) => Ok(ident.ident.clone()),
            _ => Err(Error::new_spanned(ty, "Unsupported let binding")),
        },
        _ => Err(Error::new_spanned(pat, "Unsupported let binding")),
    }
}
