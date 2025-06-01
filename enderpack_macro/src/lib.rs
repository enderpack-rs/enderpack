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

fn wrap_statement(statement: Stmt) -> syn::Result<proc_macro2::TokenStream> {
    match statement {
        Stmt::Expr(expr, _semi) => Ok(quote! { add_command(#expr) }),
        _ => Err(Error::new_spanned(statement, "Unsupported item")),
    }
}
