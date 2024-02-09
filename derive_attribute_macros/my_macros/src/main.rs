use my_macros::{make_answer, my_macro1};

fn main() {
    my_macro1!();

    // TODO: Explain why this works prior to calling make_answer!() here...
    println!("The answer is: {}", answer());

    make_answer!(); // embeds function 'answer' in-place here...
    println!("The answer is: {}", answer());
}
