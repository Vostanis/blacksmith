use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Lit, LitInt, Ident, Type, parenthesized, parse::{ParseStream, Parse, Result}, parse_macro_input, punctuated::Punctuated, Token};

//// EXAMPLE
////     #[header($NAME, $VALUE)]
////     $IDENT.get_vec($URLS, $SAVE_PATH, $THREADS).await;
////
//// METHOD
////    After parsing, reprint the "ident.get_vec(...).await;" item, now
////    surrounded by .insert(...) and .remove(...) (respectively).
////    These insert/remove to/from the HeaderMap; 
////    one of the elements of the Runner struct.
////

#[proc_macro_attribute]
pub fn header(attr: TokenStream, item: TokenStream) -> TokenStream {

    // 1. ensure 2 strings as args
    let args = Args::return_vec_of_2(parse_macro_input!(attr as Args));
    let name = &args[0];
    let value = &args[1];

    // 2. find identity of method call
    let item_clone = item.clone();
    let expr = parse_macro_input!(item_clone as MethodCall);
    let ident = &expr.ident;

    // 3. insert & remove the header, surrounding the fn_call respectively
    let item2 = parse_macro_input!(item as syn::Expr);
    quote! {
        #ident.headers.insert(#name, reqwest::header::HeaderValue::from_static(#value));
        #item2;
        #ident.headers.remove(#name);
    }.into()
}

// EXAMPLE
//     #[threads($LITERAL_INT)]
//     $IDENT.get_vec($URLS, $SAVE_PATH).await;
//

#[proc_macro_attribute]
pub fn requests(attr: TokenStream, item: TokenStream) -> TokenStream {

    // 1. ensure 1 integer
    let args = parse_macro_input!(attr as LitInt);

    // 2. find identity of method call
    let item_clone = item.clone();
    let expr = parse_macro_input!(item_clone as MethodCall);
    let ident = &expr.ident;

    // 3. change requests n of API struct
    let item2 = parse_macro_input!(item as syn::Expr);
    quote! {
        #ident.n = #args;
        #item2;
    }.into()
}

#[proc_macro_attribute]
pub fn seconds(attr: TokenStream, item: TokenStream) -> TokenStream {

    // 1. ensure 1 integer
    let args = parse_macro_input!(attr as LitInt);

    // 2. find identity of method call
    let item_clone = item.clone();
    let expr = parse_macro_input!(item_clone as MethodCall);
    let ident = &expr.ident;

    // 3. change time t of API struct
    let item2 = parse_macro_input!(item as syn::Expr);
    quote! {
        #ident.t = #args;
        #item2;
    }.into()
}

#[derive(Clone, Debug)]
struct MethodCall {
    _attrs: Vec<Attribute>,
    ident: Ident,
    _expr_ident: Ident,
    _param: Punctuated<Type, Token![,]>,
}

impl Parse for MethodCall {
    fn parse(input: ParseStream) -> Result<Self> {
        let _attrs: Vec<Attribute> = input.call(Attribute::parse_outer)?;
        let ident: Ident = input.parse()?;
        input.parse::<Token![.]>()?;
       let _expr_ident = input.parse()?;
        let content; parenthesized!(content in input);
        let _param = content.parse_terminated(Type::parse, Token![,])?; // This needs to Lit::parse or Type::parse
        input.parse::<Token![.]>()?;
        input.parse::<Token![await]>()?;
        Ok(MethodCall {
            _attrs,
            ident,
            _expr_ident,
            _param,
        })
    }
}

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

//// TO DO!
////    Below is workings for TypeExt, to allow .get() to use both Idents & Lits
////    Currently, only Ident is under the "Type::parse".
////    However, below encounters a buffer overflow.
////    Maybe there is an example somewhere this covers, e.g. within syn git or rust docs
//// 

// #[derive(Debug, Clone)]
// enum TypeExt {
//     Type(Type),
//     Lit(Lit),
// }

// impl Parse for TypeExt {
//     fn parse(input: ParseStream) -> Result<Self> {
//         Ok(input.parse()?)
//     }
// }
