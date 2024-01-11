mod pattern;

extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use pattern::Pattern;
use proc_macro::TokenStream;
use quote::quote;

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
    quote!(
        pub fn #parse_fn_name<T> (mut input: T) -> Option<u64> where T: Iterator<Item=char> {
            #pattern
        }
    )
    .into()
}
