// src/my_macros/lib.rs
// Everthing in this file is referered to as "my_macros_lib"
// because Cargo.toml [lib] name = "my_macros_lib" (in which, it's path=<this file>)

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// Declare the procedural macro dependency
extern crate proc_macro;

//use my_module2;
//pub use my_module2::my_macro;


// Interesting concept based on error:
// > error: functions tagged with `#[proc_macro]` must currently reside in the root of the crate
//#[proc_macro_derive]
//#[proc_macro_attribute]
#[proc_macro]
pub fn my_macro1(_input: TokenStream) -> TokenStream {
    // Your procedural macro logic here
    // ...
    TokenStream::new()
}


// Example usage:
//      extern crate proc_macro_examples;
//      use proc_macro_examples::make_answer;
//      
//      make_answer!();
//      
//      fn main() {
//          println!("{}", answer());
//      }
#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

// I want to define derive macros in a submodule, note that I would declare 
// it as 'my_proc_macro' but the method-function is 'my_proc_macro_fn'
// just to show that the name of the function doesn't need to match the name of the macro
#[proc_macro_derive(my_proc_macro)]
pub fn my_proc_macro_fn(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        // ...
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}