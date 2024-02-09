use my_macros::{make_answer, my_macro1, my_proc_macro};

// deriving proc-macro impl
#[derive(my_proc_macro)]
struct MyStruct {
    my_field: u32,
}

// calling proc-macro directly
pub(crate) fn do_something() {
    my_macro1!();
    // construct a 'fn answer()' function:
    make_answer!();
    println!("The answer is: {}", answer());
}   