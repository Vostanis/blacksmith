#![feature(stmt_expr_attributes)]
#[allow(dead_code)]
#[allow(unused_imports)]

// use std::collections::HashSet as Set;
use proc_macro::{TokenStream, 
    // TokenTree, Punct, Literal
};
use quote::quote;
use syn::{
    parse_macro_input, 
    Token, 
    // ExprMethodCall, 
    Lit,
    // Meta,
    parse::{ParseStream, Parse, Result},
    punctuated::Punctuated,
};

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



#[proc_macro_attribute]
pub fn header2(attr: TokenStream, item: TokenStream) -> TokenStream {

    let args = Args::return_vec_of_2(parse_macro_input!(attr as Args));
    let name = &args[0];
    let val = &args[1];
    println!("{:?}, {:?}", name, val);
    
    // let clone = item.clone();
    // let parsed = parse_macro_input!(clone as ExprMethodCall);
    item
}









    // let header: (String, String) = { 
    //     let mut attrs_iter = attr.into_iter();
    //     let name = expect_literal(&mut attrs_iter).to_string();
    //     let _ = expect_punct(&mut attrs_iter, ',');
    //     let value = expect_literal(&mut attrs_iter).to_string();
    //     (name, value)
    // };

// parsing attributes 1 by 1 as iterator (see "parse arguments at top")
// fn expect_literal(lex: &mut impl Iterator<Item=TokenTree>) -> Literal {
//     match lex.next() {
//         Some(TokenTree::Literal(literal)) => literal,
//         Some(_token) => panic!("expected Literal token but got something else"),
//         None => panic!("expected literal but got nothing"),
//     }
// }
//
// fn expect_punct(lex: &mut impl Iterator<Item=TokenTree>, ch: char) -> Punct {
//     match lex.next() {
//         Some(TokenTree::Punct(punct)) => if punct.as_char() == ch {
//             punct
//         } else {
//             panic!("Expected punctuation {expected}, but got {actual}",
//                    expected = ch, actual = punct.as_char())
//         },
//         Some(_token) => panic!("Expected punct got something else"),
//         None => panic!("Expected punct but got nothing")
//     }
// }

// fn expect_ident(lex: &mut impl Iterator<Item=TokenTree>) -> Ident {
//     match lex.next() {
//         Some(TokenTree::Ident(ident)) => ident,
//         Some(_) => panic!("Expected ident but got something else"),
//         None => panic!("Expected ident but got nothing")
//     }
// }
