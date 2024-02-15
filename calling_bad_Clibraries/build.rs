extern crate bindgen;
extern crate cc;
extern crate clang;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use cc::Build;

const EXTERN_LIB_FILENAME: &str = "bindings";    // used for lib and rs filename i.e. "bindings.rs" and "libbindings.a"

// SEE: https://doc.rust-lang.org/cargo/reference/build-scripts.html for available "cargo:" commands
// NOTE: Just like writing a BASH script or Makefile, ORDER of "cargo:" commands matters
fn main() {
    build_single_clib();
    bind_gen_clibs();
    println!("cargo:rerun-if-changed=build.rs")
}

fn for_debug_dump_outdir(debug_header: &str, out_dir: &str) {
    println!("\n\n##############################");
    // do 'ls -lAh' on the output directory to see what's there:
    if let Ok(entries) = std::fs::read_dir(&out_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                println!(
                    "# {} - File: '{}' ({})",
                    debug_header,
                    entry.file_name().to_string_lossy(),
                    out_dir,
                );
                if entry.path().is_dir() {
                    for sub_entry in entry.path().read_dir().unwrap() {
                        if let Ok(sub_entry) = sub_entry {
                            println!(
                                "# >>> {} - Sub-File: '{}/{}'",
                                debug_header,
                                entry.path().to_string_lossy(),
                                sub_entry.file_name().to_string_lossy(),
                            );
                        }
                    }
                }
            }
        }
    } else {
        eprintln!(
            "build.rs::build_single_clib() - Error reading output directory '{:?}'",
            out_dir
        );
    }
    println!("##############################\n\n");
}

// NOTE: Currently, there are ONLY ONE C library to build and bind, this will need to be
// adjusted if/when there are multiple C files to be built and bound
// Despite possibilities of having multiple C/C++ files that needs to be built, in the
// end, there should be only ONE binary library (either static .a or dynamic .so/.dll).
// Bindgen can bind to multiple header files (for 'extern "C"' block) but it's assumed
// it's only for one binary (at least for this purpose of this example project)
fn build_single_clib() {
    println!("# INFO: build_single_clib(entry)");

    let src_file = "mid_exit";
    let src_dir = "./bad_c_libs";

    let out_dir = env::var("OUT_DIR").unwrap();
    println!("# INFO: OUT_DIR: {:?}", out_dir);
    // show current directory so that we know whether the path for "rerun-if-changed" is correct:
    println!(
        "# INFO: Current directory: {:?}",
        std::env::current_dir().unwrap_or_else(|err| {
            eprintln!(
                "build.rs::build_single_clib() - Error getting current directory: {}",
                err
            );
            std::process::exit(1);
        })
    );
    // Undocumented assumptions: path specified is *NIX based even on Windows target!
    println!("cargo:rerun-if-changed=./{}/{}.c", src_dir, src_file);
    println!("cargo:rerun-if-changed=./{}/{}.h", src_dir, src_file);

    // $ _LIB_SRC=mid_exit
    // $ #clang -v -c -target x86_64-pc-windows-gnu -o ${_LIB_SRC}.o ${_LIB_SRC}.c
    // $ clang -v -c -o ${_LIB_SRC}.o ${_LIB_SRC}.c
    // $ # create the library
    // $ ar rcs lib${_LIB_SRC}.a ${_LIB_SRC}.o
    //Command::new("clang")
    //    .args(&[format!("{}/{}.c", src_dir, src_file), "-c".into(), "-v".into(), "-fPIC".into(), "-o".into()])
    //    .arg(&format!("{}/{}.o", out_dir, src_file))
    //    .status()
    //    .unwrap();
    // Make sure to have "clang" crate in Cargo.toml [build-dependencies], so we can use clang::Build() instead
    // NOTE: when using cc::Build::compile() or cc::Build::try_compile(), the output file name is automatically
    //       set to "lib<src_file>.a" (static file), hence if you want to have a separate shared library, you
    //       WILL NEED to recompile again with .shared_flag().  Alternatively, don't use cc::Build and
    //       just cc/clang to generate the .o file...
    let cc_result = cc::Build::new() // USE gcc (should be OK, as long as cc crate is installed)
        //.compiler("clang")    // USE clang
        .file(format!("{}/{}.c", src_dir, src_file))
        .flag("-c") // compile only
        .flag("-v") // verbose (use -vv for very-verbose)
        .flag("-fPIC") // position independent code (PIC), commonly used for shared libraries (.so/.dll)
        .flag("-Werror") // treat ALL warnings as errors
        .flag("-Wall") // enable ALL warnings
        .flag("-Wno-error=div-by-zero") // disable warning for division by zero (because this library needs to demonstrate divide-by-zero is not going to cause damage)
        //.static_flag(true)
        .try_compile(EXTERN_LIB_FILENAME); // apparently, the crate knows about the OUT_DIR
    if cc_result.is_err() {
        // do our own form of panic!()/fail!() here
        eprintln!(
            "build.rs::build_single_clib() - Error compiling C source file: {:?}",
            cc_result.err()
        );
        std::process::exit(1);
    }
    println!(
        "# INFO: build_single_clib() - compiled output: '{}/{}.a'",
        out_dir, src_file
    );

    //// ### NOTE: We do NOT need to do "ar" if we used cc::Build::compile() since it automatically will implicitly create the static archive (.a) file.
    //println!(
    //    "# INFO: build_single_clib() - using `ar` to create static archive: '{}/lib{}.a'",
    //    out_dir, src_file
    //);
    //Command::new("ar")
    //    .args(&[
    //        "crus".into(),
    //        format!("lib{}.a", src_file),
    //        format!("{}/{}.o", src_dir, src_file),    // cc::Build::compile() will automatically create this .o file in OUT_DIR/src_dir for preservations
    //    ])
    //    .current_dir(&Path::new(&out_dir))
    //    .status()
    //    .unwrap(); // hopefully, panic message here is meaningful enough...

    //// Build shared object (.so/.dll) (requires recompile)
    //println!(
    //    "# INFO: build_single_clib() - converting '{}/{}/{}.o' to shared object '{}/{}.so' (or .dll)",
    //    out_dir, src_dir, src_file, out_dir, src_file
    //);
    //let to_shared_lib = cc::Build::new() // USE gcc (should be OK, as long as cc crate is installed)
    //    //.compiler("clang")    // USE clang
    //    .shared_flag(true) // create shared library
    ////    .file(format!("{}/{}.c", src_dir, src_file))    // note that usually, we do 'gcc -shared -o <src_file>.so <src_file>.o' (note that it's binary-to-binary from .o file)
    //    .file(format!("{}/{}.o", out_dir, src_dir, src_file))
    //    .flag("-c") // compile only
    //    .flag("-v") // verbose (use -vv for very-verbose)
    //    .flag("-fPIC") // position independent code (PIC), commonly used for shared libraries (.so/.dll)
    //    .flag("-Werror") // treat ALL warnings as errors
    //    .flag("-Wall") // enable ALL warnings
    //    .flag("-Wno-error=div-by-zero") // disable warning for division by zero (because this library needs to demonstrate divide-by-zero is not going to cause damage)
    //    .try_compile(EXTERN_LIB_FILENAME); // apparently, the crate knows about the OUT_DIR - hopefully, this knows how to set extension as either .so or .dll
    //if to_shared_lib.is_err() {
    //    // do our own form of panic!()/fail!() here
    //    eprintln!(
    //        "build.rs::build_single_clib() - Error creating shared object: {:?}",
    //        to_shared_lib.err()
    //    );
    //    std::process::exit(1);
    //}

    for_debug_dump_outdir("build_single_clib()", &out_dir); //////////////////////////////////////////////////////////////////////////

    // ############################## BEGIN LINKER:
    // NOTE: from what I understand, 'rustc' compiler will bias toward dynamic linking if all is found, but at the same time,
    // it has the "if found" logic in which you can define kind "dylib" or "static", or both of same lib-name.
    // make sure to add the library search path to the linker
    println!("cargo:rustc-link-search=native={}", out_dir); // in native

    // Specify the library search path
    println!("cargo:rustc-link-search={}", out_dir); // just in case, mixed

    // Link the native library dynamically (dylib, as in *.so and/or *.dll)
    println!("cargo:rustc-link-lib=dylib={}", EXTERN_LIB_FILENAME);

    // Link the static library explicitly
    println!("cargo:rustc-link-lib=static={}", EXTERN_LIB_FILENAME);

    // just let rustc figure out the best way to link the library
    println!("cargo:rustc-link-lib={}", EXTERN_LIB_FILENAME);

    // ############################## END LINKER
}

fn bind_gen_clibs() {
    println!("# INFO: bind_gen_clibs(entry)");
    println!("cargo:rerun-if-changed=./bad_c_libs/mid_exit.h");
    println!("cargo:rerun-if-changed=./bad_c_libs/mid_exit.c");

    // Define your header file paths here (either Vec<String> or array of strings)
    let header_paths: Vec<String> = vec![
        "./bad_c_libs/mid_exit.h".to_string(),
        // Add more header paths as needed
    ];

    // Accumulate all bindings
    let mut all_bindings = String::new();

    for header_path in &header_paths {
        let bindings = bindgen::Builder::default()
            .header(header_path)
            .generate()
            .expect("build.rs::bind_gen_clibs() - Unable to generate bindings");

        all_bindings.push_str(&bindings.to_string());
    }

    // Write all bindings to a single file
    let dest_src_name = format!("{}.rs", EXTERN_LIB_FILENAME);      // i.e. "bindings.rs"
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(out_dir.clone());
    let bindings_file = out_path.join(dest_src_name);
    std::fs::write(&bindings_file, all_bindings)
        .expect("build.rs::bind_gen_clibs() - Couldn't write bindings!");

    for_debug_dump_outdir("bind_gen_clibs()", &out_dir.clone().as_str()); //////////////////////////////////////////////////////////////////////////
}
