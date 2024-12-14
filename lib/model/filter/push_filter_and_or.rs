use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::Ident;

pub fn push_filter_and_or(
    f: &Ident,
    fields: &mut Vec<TokenStream2>,
    matches: &mut Vec<TokenStream2>,
    op_str: &str,
) {
    let op = format_ident!("{}", op_str);
    let graphql_name_str = op_str.to_uppercase();
    let condition_str = if op_str == "and" { "all" } else { "any" };
    let condition = format_ident!("{}", condition_str);

    fields.push(quote! {
        #[graphql(name=#graphql_name_str)]
        pub #op: Option<Vec<#f>>,
    });
    matches.push(quote! {
        if let Some(v) = this.#op {
            let mut #op = Condition::#condition();
            for f in v {
                #op = #op.add(f.condition());
            }
            c = c.add(#op);
        }
    });
}
