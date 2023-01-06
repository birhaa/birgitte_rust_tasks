extern crate proc_macro;
use std::collections::HashMap;
use proc_macro::*;
use lazy_static::lazy_static;


lazy_static! {
    /// This is an example for using doc comment attributes
    static ref CACHE : HashMap<usize, usize> = HashMap::new();
}

#[proc_macro_attribute]
pub fn my_custom_memoization(_attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", _attr.to_string());
    println!("item: \"{}\"", item.to_string());

    // let mut item: syn::Item = syn::parse(item).unwrap();
    // let fn_item = match &mut item {
    //     syn::Item::Fn(fn_item) => fn_item,
    //     _ => panic!("expected fn")
    // };

    item
}