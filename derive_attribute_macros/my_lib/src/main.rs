// src/main.rs (my_lib/main.rs)

// Import submodule1
mod my_module1;
use my_module1::sub_module1;

// I want to use my procedural macro in my [[bin]]
use ::my_macros::my_macro1;

fn main() {
    sub_module1::do_something();
    my_macro1!(); // Invoke the procedural macro
}
