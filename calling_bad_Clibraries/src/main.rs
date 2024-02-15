mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn main() {
    // access the C function from the generated bindings
    println!("main(0): About to call mid_exit() from C library");
    let status = -666;
    unsafe {
        ffi::mid_exit(status);
    }
    println!("main(1): mid_exit() from C library called successfully");
}
