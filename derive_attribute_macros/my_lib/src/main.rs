// src/main.rs (my_lib/main.rs)

// Import submodule1
mod my_module1;
use my_macros::make_answer;
use my_module1::sub_module1;

// I want to use my procedural macro in my [[bin]]
use ::my_macros::my_macro1;

fn main() {
    // Uncomment below to verify that the procedural macro is not yet available...
    // actually, if you call make_answer!() way at the end of this function, you still can call answer() here...
    //println!("The answer is: {}", answer());

    sub_module1::do_something();    // internally will call answer() just as a test...
    my_macro1!(); // Invoke the procedural macro

    //make_answer!(); // creates a method answer() but it is called way up above...
}
