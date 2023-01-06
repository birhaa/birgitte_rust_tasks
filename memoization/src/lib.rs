extern crate proc_macro;
use std::collections::HashMap;
use proc_macro::*;
use lazy_static::lazy_static;

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref CACHE : HashMap<usize, usize> = HashMap::new();
}

//This is not used, was just curious of how it works with custom macro attributes in rust
#[proc_macro_attribute]
pub fn my_custom_memoization(_attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", _attr.to_string());
    println!("item: \"{}\"", item.to_string());

    item
}