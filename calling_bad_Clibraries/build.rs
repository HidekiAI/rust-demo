extern crate bindgen;

use bindgen::Builder;
use std::env;
use std::path::PathBuf;

fn main() {
    // Define your header file paths here (either Vec<String> or array of strings)
    let header_paths: Vec<String> = vec![
        "./bad_c_libs/mid_exit.h".to_string(),
        // Add more header paths as needed
    ];

    // ############################## BEGIN LINKER:
    // Specify the library search path
    println!("cargo:rustc-link-search=native=./bad_c_libs/");

    // Link the native library dynamically (dylib, as in *.so and/or *.dll)
    //println!("cargo:rustc-link-lib=dylib=mylib");

    // Link the static library explicitly
    println!("cargo:rustc-link-lib=static=mid_exit");

    // ############################## END LINKER

    // Accumulate all bindings
    let mut all_bindings = String::new();

    for header_path in &header_paths {
        let bindings = bindgen::Builder::default()
            .header(header_path)
            .generate()
            .expect("Unable to generate bindings");

        all_bindings.push_str(&bindings.to_string());
    }

    // Write all bindings to a single file
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_file = out_path.join("bindings.rs");
    std::fs::write(&bindings_file, all_bindings).expect("Couldn't write bindings!");
}
