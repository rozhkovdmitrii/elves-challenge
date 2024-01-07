mod pattern;

extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use pattern::Pattern;
use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

fn gen_impl(pattern: Pattern, depth: usize) -> proc_macro2::TokenStream {
    match pattern {
        Pattern::Check(to_be_matched) => {
            let key = to_be_matched
                .keys()
                .map(|key| syn::LitChar::new(*key, key.span()));
            let value = to_be_matched.values().map(|pattern| match pattern {
                Pattern::Result(value) => quote!( Some(#value) ),
                pattern => gen_impl(pattern.clone(), depth + 1).into(),
            });
            quote!(
                match input.next()? {
                    #(#key => #value),*,
                    _ => None
                }
            )
            .into()
        }
        Pattern::Result(target) => quote!( #target, ).into(),
    }
}

#[proc_macro]
pub fn gen_digit_parser(item: TokenStream) -> TokenStream {
    let mut tokens = item.into_iter();
    let parse_fn_name: syn::Ident = syn::parse(
        tokens
            .next()
            .expect("Expected parse fn name to be provided")
            .into(),
    )
    .expect("Expected parse fn name to be parsed well");

    let pattern = Pattern::build_from_token_stream(tokens);
    let fwd_match_code = gen_impl(pattern, 0);

    quote!(
        pub fn #parse_fn_name<T> (mut input: T) -> Option<u64> where T: Iterator<Item=char> {
            #fwd_match_code
        }
    )
    .into()
}
