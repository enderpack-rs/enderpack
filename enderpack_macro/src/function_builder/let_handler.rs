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
        name: &Ident,
        ty: &Type,
        local_init: &LocalInit,
    ) -> syn::Result<Vec<TokenStream2>> {
        Ok[quote! {}]
    }
}
