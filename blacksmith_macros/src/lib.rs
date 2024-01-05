use proc_macro::{TokenStream, TokenTree, Punct, Literal};
// use quote::quote;
// use syn::{parse2};
use serde::{Serialize, Deserialize};

static mut STAMPED_FNS: Vec<FnStr> = Vec::new();

#[proc_macro_attribute]
pub fn header(attrs: TokenStream, item: TokenStream) -> TokenStream {

    // expected attribute-input should be in the form: #[header(str1, str2)]
    // parse attribute arguments
    let _header: String = { 
        let mut attrs_iter = attrs.into_iter();
        let name = expect_literal(&mut attrs_iter).to_string();
        let _ = expect_punct(&mut attrs_iter, ',');
        let value = expect_literal(&mut attrs_iter).to_string();
        format!(".header({}, {})", name, value)
    };

    // find and retrieve file of function called; as text
    // replace any ".send()" with ".header(#name, #value).send()"
    // module_path!() //run with empty brackets
    // include_str!("file_name.txt")
    // 2 paths:
    //      1. initial call of get_vec() => use the static str
    //          |_ if there is no mention of "pub async fn"
    //      2. sequential call of get_vec() => use the item
    //          |_ else ...
    //
    // parse function and insert .header(name, value)
    fn find_fn_name(fn_string: String) -> String {
        let phrases: Vec<&str> = item_str.split("::").collect();
        for phrase in phrases {
            match phrase {
               
            }
        }
        // for phrase in phrases.iter() {
        //     if phrase.contains("(") && phrase.contains(")") {
        //         let bracket = phrase.find("(").expect("Couldn't find opening parenthesis") - 1;
        //         &phrase.trim()[0..bracket]
        //     } else {
        //         ""
        //     };
        // };
        ident
    }

    let fn_name = {
        let item_str = item.to_string();
        let phrases: Vec<&str> = item_str.split("::").collect();

        // find the final phrase in eg::eg::example().await => example().await is the final
        // phrase
        // let mut ident = "";
        let ident = for phrase in phrases.iter() {
            if phrase.contains("(") && phrase.contains(")") {
                let bracket = phrase.find("(").expect("Couldn't find opening parenthesis") - 1;
                &phrase.trim()[0..bracket]
            } else {
                ""
            };
        };
        ident
    };
    println!("{fn_name:?}"); // DIDN'T WORK

    guarantee_json_file("collected_fns.json");
    let collect_fns: Vec<FnStr> = read_json_file::<Vec<FnStr>>("./collected_fns.json");
    println!("{collect_fns:#?}");

    item
    // return new item fn with call to fn
    // quote! { 
    //     // #modified_itemfn
    //     // get_vec(urls, "./src", 3).await;
    //     todo!()
    // }.into()
}

static mut STAMP_COUNTER: u16 = 0;

#[proc_macro_attribute]
pub fn stamped_for_collection(_attrs: TokenStream, item: TokenStream) -> TokenStream {

    // find func name, and store with function
    let func: FnStr = {
        let item_str = item.to_string();
        let fn_name = {
            if item_str.contains("fn ") || item_str.contains("fn\n") {
                let id_start = item_str.find("fn").expect("Failed to find \"fn\" string") + 3;
                let id_end = item_str.find("(").expect("Failed to find open bracket");
                item_str[id_start..id_end].to_string()
            } else {
                panic!("Item called does not look like a function definition: {}", item_str);
            }
        }; 
        FnStr {
            fn_name: fn_name,
            item_str: item_str,
        }
    };
    
    // println!("{func:#?}");
    // unsafe { STAMPED_FNS.push(func); }
    unsafe {
        // delete the file
        if STAMP_COUNTER == 0 { 
            use std::fs::OpenOptions;
            use std::io::{Seek, SeekFrom, Write};

            let _ = std::fs::remove_file("collected_fns.json");

            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .open("collected_fns.json")
                .expect("Unable to open file");

            file.seek(SeekFrom::Start(0)).expect("Unable to seek beginning of file");
            file.write_all("[".as_bytes()).expect("Failed to write bytes to file");
        }
    }

    // insert FnStr
    append_json_file("collected_fns.json", func);
    unsafe { STAMP_COUNTER += 1; }

    // return the item (unedited)
    item
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FnStr {
    fn_name: String,
    item_str: String,
}

// read json file to type T (that derives Deserialize)
fn read_json_file<T: serde::de::DeserializeOwned>(file_path: &str) -> T {
    use std::io::Read;
    let mut file = std::fs::File::open(file_path).expect("Cannot open file");
    let mut file_str = String::new();
    file.read_to_string(&mut file_str).expect("Cannot read to string");
    let json: T = serde_json::from_str(&file_str).expect("BLAH BLAH BLAH");
    json
}

fn append_json_file(file_path: &str, collected_fn: FnStr) {
    use std::fs::OpenOptions;
    use std::io::{Seek, SeekFrom, Write};

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_path)
        .expect("Unable to open file");

    file.seek(SeekFrom::End(0)).expect("Unable to seek end of file");
    let mut json = serde_json::to_string(&collected_fn).expect("Failed to stringify fn");
    json.push_str(",");

    file.write_all(json.as_bytes()).expect("Failed to write bytes to file");
}

fn guarantee_json_file(file_path: &str) {
    use std::fs::OpenOptions;
    use std::io::{Seek, SeekFrom, Write};

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_path)
        .expect("Unable to open file");

    // file.seek(SeekFrom::Start(1)).expect("Unable to seek beginning of file");
    // file.write_all("[".as_bytes()).expect("Failed to write bytes to file");
    
    file.seek(SeekFrom::End(-1)).expect("Unable to seek beginning of file");
    file.write_all("]".as_bytes()).expect("Failed to write bytes to file");
}

// parsing attributes 1 by 1 as iterator (see "parse arguments at top")
fn expect_literal(lex: &mut impl Iterator<Item=TokenTree>) -> Literal {
    match lex.next() {
        Some(TokenTree::Literal(literal)) => literal,
        Some(_token) => panic!("expected Literal token but got something else"),
        None => panic!("expected literal but got nothing"),
    }
}

fn expect_punct(lex: &mut impl Iterator<Item=TokenTree>, ch: char) -> Punct {
    match lex.next() {
        Some(TokenTree::Punct(punct)) => if punct.as_char() == ch {
            punct
        } else {
            panic!("Expected punctuation {expected}, but got {actual}",
                   expected = ch, actual = punct.as_char())
        },
        Some(_token) => panic!("Expected punct got something else"),
        None => panic!("Expected punct but got nothing")
    }
}

















//
// // static Vec with (function identity, function item) [static hash maps are more annoying]
// static mut STAMPED_FOR_COLLECTION: Vec<(String, String)> = Vec::new();
//
// #[proc_macro]
// pub fn init_stamped(_item: TokenStream) -> TokenStream {
//      let STAMPED_FOR_COLLECTION: Vec<CollectFnString> = Vec::new();
// }
//
// #[derive(Debug)]
// struct CollectedFnString {
//     name: String,
//     item_str: String,
// }
//
// // add every stamped func definition to STR_STAMPED_VEC (with a name to recall)
// #[proc_macro_attribute]
// pub fn stamped_for_collection(_attrs: TokenStream, item: TokenStream) -> TokenStream {
//
//     // find func name, and store with function
//     let _func: CollectedFnString = {
//         let item_str = item.to_string();
//         let fn_name = {
//             if item_str.contains("fn ") || item_str.contains("fn\n") {
//                 let id_start = item_str.find("fn").expect("Failed to find \"fn\" string") + 3;
//                 let id_end = item_str.find("(").expect("Failed to find open bracket");
//                 item_str[id_start..id_end].to_string()
//             } else {
//                 panic!("Item called does not look like a function definition: {}", item_str);
//             }
//         };
//
//         FnStr {
//             name: fn_name,
//             item_str: item_str,
//         }
//     };
//
//      println!("{_func:?}");
//     
//     // // push to static vector
//     // unsafe {
//     //     STAMPED_FOR_COLLECTION.push(func.clone());
//     //     println!("{:?}", &func);
//     // }
//     item
// }
//
// #[proc_macro_attribute]
// pub fn header(attrs: TokenStream, item: TokenStream) -> TokenStream {
//
//     // expected attribute-input should be in the form: #[header(str1, str2)]
//     // parse attribute arguments
//     let header: String = { 
//         let mut attrs_iter = attrs.into_iter();
//         let name = expect_literal(&mut attrs_iter).to_string();
//         let _ = expect_punct(&mut attrs_iter, ',');
//         let value = expect_literal(&mut attrs_iter).to_string();
//         format!(".header({}, {})", name, value)
//     };
//
//     // find and retrieve file of function called; as text
//     // replace any ".send()" with ".header(#name, #value).send()"
//     // module_path!() //run with empty brackets
//     // include_str!("file_name.txt")
//     // 2 paths:
//     //      1. initial call of get_vec() => use the static str
//     //          |_ if there is no mention of "pub async fn"
//     //      2. sequential call of get_vec() => use the item
//     //          |_ else ...
//     //
//     // parse function and insert .header(name, value)
//     let modified_itemfn = {
//         let item_str = item.to_string();
//         let phrases: Vec<&str> = item_str.split("::").collect();
//
//         // find the final phrase in eg::eg::example().await => example().await is the final
//         // phrase
//         let mut ident = "";
//         for phrase in phrases {
//             if phrase.contains("(") && phrase.contains(")") {
//                 let bracket = phrase.find("(").expect("Couldn't find opening parenthesis") - 1;
//                 ident = &phrase.trim()[0..bracket];
//                 break;
//             };
//         } 
//
//         let mut item_str = String::new();
//         unsafe {
//             println!("{}", STAMPED_FOR_COLLECTION.is_empty());
//             for (i, _idents) in STAMPED_FOR_COLLECTION.iter().enumerate() {
//                 println!("THIS AND THAT: {} : {}", STAMPED_FOR_COLLECTION[i].0, STAMPED_FOR_COLLECTION[i].1);
//                 // if ident == STAMPED_FOR_COLLECTION[i].0 {
//                 //     item_str = STAMPED_FOR_COLLECTION[i].1.clone();
//                 //     break;
//                 // };
//             }
//         }
//
//         // find ".send()" and insert ".header(#name, #value)" before it
//         println!("ITEM STRING: {item_str}");
//         // let send_method = item_str.find(".send()")
//         //     .expect("expected to find \"send()\" function. Function should include ");
//         // item_str.insert_str(send_method, &header);
//         // parse_str::<ItemFn>(&item_str).expect("failed to parse item string as ItemFn");
//  
//         println!("{ident}");
//         item
//     };
//
//     // return new item fn with call to fn
//     quote! { 
//         // #modified_itemfn
//         // get_vec(urls, "./src", 3).await;
//         todo!()
//     }.into()
// }
//

