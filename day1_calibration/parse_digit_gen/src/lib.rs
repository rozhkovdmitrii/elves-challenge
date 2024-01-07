extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use std::collections::BTreeMap;
use syn::spanned::Spanned;
use syn::{LitInt, LitStr};

#[derive(Clone, Debug)]
enum Pattern {
    Result(u64),
    Check(BTreeMap<char, Pattern>),
}

impl Pattern {
    fn with(mut self, digit: &str, value: u64) -> Self {
        Self::extend_pattern(digit, value, &mut self);
        self
    }

    fn extend_pattern(digit: &str, value: u64, pattern: &mut Pattern) {
        let Some(first) = digit.chars().nth(0) else {
            *pattern = Pattern::Result(value);
            return;
        };
        match pattern {
            Pattern::Result(_) => panic!("Unexpected case while constructing new pattern route"),
            Pattern::Check(ref mut rules) => {
                let mut pattern = rules
                    .entry(first)
                    .or_insert_with(|| Pattern::Check([].into()));
                Self::extend_pattern(&digit[1..], value, &mut pattern);
            }
        };
    }

    fn dec_radix_based() -> Self {
        Pattern::default()
            .with("0", 0)
            .with("1", 1)
            .with("2", 2)
            .with("3", 3)
            .with("4", 4)
            .with("5", 5)
            .with("6", 6)
            .with("7", 7)
            .with("8", 8)
            .with("9", 9)
    }

    fn build_from_token_stream<T>(tokens: T) -> Pattern
    where
        T: Iterator<Item = TokenTree>,
    {
        let mut rule = (None, None);
        let mut pattern = Pattern::dec_radix_based();
        for token in tokens {
            if let Ok(token) = syn::parse::<LitStr>(token.clone().into()) {
                rule.0 = Some(token.value());
            } else if let Ok(token) = syn::parse::<LitInt>(token.into()) {
                rule.1 = Some(
                    token
                        .base10_parse()
                        .expect("Expected token to be parsed as number well"),
                );
            };
            if rule.0.is_some() && rule.1.is_some() {
                pattern = pattern.with(
                    rule.0
                        .take()
                        .expect("Expected pattern rule to be set previously")
                        .as_str(),
                    rule.1.unwrap(),
                );
                rule = (None, None)
            }
        }
        pattern
    }
}

impl Default for Pattern {
    fn default() -> Self {
        Pattern::Check(BTreeMap::default())
    }
}

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
