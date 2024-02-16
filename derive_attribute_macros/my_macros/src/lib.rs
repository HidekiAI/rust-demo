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
    // just like traditional C/C++ macro, a "macro" in this case
    // will place this string IN PLACE of the macro call as
    // a function "fn answer()"
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

// Purpose of this macro:
// verifies that the struct has a field named `my_var1` of type `f32`
// and then generates a method to halve `my_var1` as `half_my_var1` in which
// the caller can assume that they will inherit that function without impl it themselves
#[proc_macro_derive(MyProcMacro)]
pub fn my_proc_macro_fn(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Check if the struct has a field named "my_var1" of type "f32"
    let my_var1_exists = match input.data {
        syn::Data::Struct(ref data) => data.fields.iter().any(|field| {
            field
                .ident
                .as_ref()
                .map_or(false, |ident| ident == "my_var1")
                && matches!(&field.ty, syn::Type::Path(p) if p.path.is_ident("f32"))
        }),
        _ => false,
    };

    if !my_var1_exists {
        // Emit a helpful compiler error
        return syn::Error::new_spanned(
            &input,
            "Struct must have a field named 'my_var1' of type 'f32'",
        )
        .to_compile_error()
        .into();
    }

    // Ensure that the struct has a field named `my_var1` of type `f32`
    let _my_var1_type = quote! { f32 };
    let struct_name = &input.ident;

    // Generate the code to halve `my_var1`
    let expanded = quote! {
        impl #struct_name {
            pub fn half_my_var1(&mut self) {
                self.my_var1 /= 2.0;
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
