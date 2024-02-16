use my_macros::{make_answer, my_macro1, MyProcMacro};

#[allow(dead_code)]
// deriving proc-macro impl
#[derive(MyProcMacro)]
struct MyStruct {
    my_field: u32,
    my_var1: f32, // NOTE: Try commenting this out, you should get the compiler warning that 'my_var1' does not exist, and if you change this to i32 (or any other type than f32), you will also get the message defined in the proc-macro
}

// calling proc-macro directly
#[allow(dead_code)] // though main.rs calls this method, I'm still getting dead_code warning...
pub(crate) fn do_something() {
    my_macro1!();
    // construct a 'fn answer()' function:
    make_answer!();
    println!("The answer is: {}", answer());

    // the MyProcMacro should have a method 'half_my_var1' that halves the value of 'my_var1'
    // first, dump my_var1 to verify its value
    let mut my_struct = MyStruct {
        my_field: 42,
        my_var1: 3.14,
    };
    println!("my_var1: {}", my_struct.my_var1);
    // then, call the method to halve my_var1
    my_struct.half_my_var1();
    println!("half_my_var1: {:?}", my_struct.my_var1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_something() {
        do_something();
        // Add assertions here to verify the expected behavior of the do_something function
    }
}
