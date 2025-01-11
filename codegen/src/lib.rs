use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, token::Comma, LitInt};

/// Add two `u32`s
///
/// Arguments:
/// - `lhs`: The left-hand side of the expression
/// - `rhs`: The right-hand side of the expression
#[proc_macro]
pub fn sum_macro(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input with Punctuated<LitInt, Comma>::parse_terminated);
    let mut iter = args.iter().map(|v| v.base10_parse::<u32>().unwrap());

    let lhs = iter.next().unwrap_or(0);
    let rhs = iter.next().unwrap_or(0);

    quote! {
        #lhs + #rhs
    }
    .into()
}

/// Add two `u32`s
///
/// Arguments:
/// - `lhs`: The left-hand side of the expression
/// - `rhs`: The right-hand side of the expression
#[proc_macro]
pub fn sum_macro_name_args_idea(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input with Punctuated<LitInt, Comma>::parse_terminated);
    let mut iter = args.iter().map(|v| v.base10_parse::<u32>().unwrap());

    let lhs = iter.next().unwrap_or(0);
    let rhs = iter.next().unwrap_or(0);

    quote! {{
        fn name_args<T, U>(lhs: T, rhs: U) -> (T, U) { (lhs, rhs) }
        let (lhs, rhs) = name_args(#lhs, #rhs);
        lhs + rhs
    }}
    .into()
}
