use proc_macro::TokenTree;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use std::collections::BTreeMap;
use syn::{spanned::Spanned, LitInt, LitStr};

#[derive(Clone, Debug)]
pub enum Pattern {
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
                let pattern = rules
                    .entry(first)
                    .or_insert_with(|| Pattern::Check([].into()));
                Self::extend_pattern(&digit[1..], value, pattern);
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

    pub(super) fn build_from_token_stream<T>(tokens: T) -> Pattern
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

impl ToTokens for Pattern {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let to_append = match self {
            Pattern::Check(to_be_matched) => {
                let key = to_be_matched
                    .keys()
                    .map(|key| syn::LitChar::new(*key, key.span()));
                let value = to_be_matched.values();
                quote!(
                    match input.next()? {
                        #(#key => #value),*,
                        _ => None
                    }
                )
            }
            Pattern::Result(target) => quote!( Some(#target) ),
        };
        tokens.append_all(to_append)
    }
}
