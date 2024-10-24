use std::env;
use std::path::PathBuf;

fn main() {
    
    // cbindgen
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Failed to generate c-bindings.")
        .write_to_file("target/rustui.h");

    println!("cargo:rerun-if-changed=src/lib.rs");

    // bindgen

    // Tell cargo to look for shared libraries in the specified directory
    //println!(
    //    "cargo:rustc-link-search=/home/charles/workspace/fire-and-security/esmi-fdp/NETB/code/ui"
    //);

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("../main.c")
        //.allowlist_file("..ui.h")
        //.allowlist_function("rustui_called_from_c")
        //.allowlist_function("rustui_called_from_c_with_args")
        //.allowlist_function("rustui_called_from_c_with_return")
        //.clang_args(vec![
        //    "-I/home/charles/workspace/fire-and-security/esmi-fdp/NETB/code",
        //    "-I/home/charles/workspace/fire-and-security/esmi-fdp/common/code",
        //    "-I/home/charles/workspace/fire-and-security/esmi-fdp/NETB/code/include",
        //])
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("netb_bindings.rs"))
        .expect("Couldn't write bindings!");
    
}
