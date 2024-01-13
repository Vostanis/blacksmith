#![feature(stmt_expr_attributes)]
#[allow(dead_code)]
#[allow(unused_imports)]

use proc_macro::TokenStream;
// use quote::quote;
use syn::{
    parse_macro_input, 
    Token,
    Lit,
    parse::{ParseStream, Parse, Result},
    punctuated::Punctuated,
};

/// Example use:
///
///     #[header($NAME, $VALUE)]
///     $IDENT.get_vec($URLS, $SAVE_PATH, $THREADS).await;
///
#[proc_macro_attribute]
pub fn header(attr: TokenStream, item: TokenStream) -> TokenStream {

    // 1. ensure 2 strings as args
    let args = Args::return_vec_of_2(parse_macro_input!(attr as Args));
    let name = &args[0];
    let val = &args[1];
    println!("{:?}, {:?}", name, val);

    // 2. find identity of method call
    let item_clone = item.clone();
    let method = parse_macro_input!(item_clone as syn::Expr);

    println!("{method:#?}");

    // 3. insert header to HeaderMap, call the fn, then remove the header
    // let clone = item.clone();
    // let parsed = parse_macro_input!(clone as ExprMethodCall);
    item
}

// parsing macro inputs
#[derive(Clone, Debug)]
struct Args {
    vars: Vec<Lit>,
} 

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let vars = Punctuated::<Lit, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            vars: vars.into_iter().collect(),
        })
    }
}

impl Args {
    fn return_vec_of_2(args: Args) -> Vec<Lit> {
        let mut output: Vec<Lit> = vec![];
        for var in args.vars {
            match var {
                Lit::Str(ref _valid) => output.push(var),
                _ => panic!("expected string variables only"),
            }
        }
        if output.len() == 2 { return output }
        else { panic!("expected 2 arguments") }
    }
}


