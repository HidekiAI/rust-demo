// Either this or create a BASH script that will call all C/C++ build.sh
// scripts scattered on different sub-modules, and then call `cargo build`,
// OR have it call it here, and let `cargo build` do its thing...
//
use std::process::Command;

fn main() {
    // $ pushd .
    // $ cd calling_bad_Clibraries/bad_c_libs/
    // $ ./build.sh
    // $ popd
    // Execute your shell script
    // Get the current directory
    let current_dir = std::env::current_dir().unwrap_or_else(|err| {
        eprintln!("Error getting current directory: {}", err);
        exit(1);
    });

    // Change to the directory where build.sh resides
    let script_dir = "calling_bad_Clibraries/bad_c_libs/";
    if let Err(err) = std::env::set_current_dir(script_dir) {
        eprintln!("Error changing directory: {}", err);
        exit(1);
    }

    // Execute build.sh
    let status = Command::new("sh")
        .arg("./build.sh")
        .status()
        .unwrap_or_else(|err| {
            eprintln!("Error executing build.sh: {}", err);
            exit(1);
        });

    if !status.success() {
        eprintln!("build.sh failed");
        exit(1);
    }

    // Return to the original directory
    if let Err(err) = std::env::set_current_dir(current_dir) {
        eprintln!("Error changing back to original directory: {}", err);
        exit(1);
    }
}
