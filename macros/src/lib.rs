//! Procedural macro for Ethereum addresses.
//!
//! See [`ethaddr`](https://docs.rs/ethaddr/latest/ethaddr/macro.address.html)
//! documentation for more information.

extern crate proc_macro;

use proc_macro::{Literal, TokenStream, TokenTree};
use std::iter::Peekable;

#[proc_macro]
pub fn address(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter().peekable();

    // Read the optional `~` prefix signalling that the address should be parsed
    // without verifying the checksum.
    let checksum = take(&mut tokens, |t| match t {
        TokenTree::Punct(p) if p.as_char() == '~' => Some(()),
        _ => None,
    })
    .is_some();

    // Read the address string literal.
    let input = take(&mut tokens, |t| match t {
        TokenTree::Literal(v) => parse_string(v),
        _ => None,
    })
    .ok_or_else(|| match tokens.peek() {
        Some(_token) => {
            // Unexpected token error
            ""
        }
        None => {
            // Unexpected end of input error
            ""
        }
    })
    .unwrap();

    if let Some(token) = tokens.next() {
        // Extrenous token error
        panic!("");
    }

    /*
    let address = if checksum {
        Address::from_str_checksum(&input)
    } else {
        input.parse()
    };
    */

    "::ethaddr::Address([0; 20])".parse().unwrap()
}

fn take<I, F, T>(tokens: &mut Peekable<I>, f: F) -> Option<T>
where
    I: Iterator<Item = TokenTree>,
    F: FnOnce(&TokenTree) -> Option<T>,
{
    let taken = f(tokens.peek()?)?;
    tokens.next();
    Some(taken)
}

fn parse_string(literal: &Literal) -> Option<String> {
    Some(
        literal
            .to_string()
            .strip_prefix('"')?
            .strip_suffix('"')?
            .to_owned(),
    )
}
