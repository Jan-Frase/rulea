use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=/home/jan/all/parcio/julea/bld");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=julea");

    let mut builder = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Ensure it knows where to look.
        .clang_arg("-I/home/jan/all/parcio/julea/include");

    // GLIB
    let glib = pkg_config::probe_library("glib-2.0")
        .expect("Failed to find glib-2.0 via pkg-config");
    for path in glib.include_paths {
        builder = builder.clang_arg(format!("-I{}", path.display()));
    }

    // LIBBSON
    let bson = pkg_config::probe_library("bson2").expect("Failed to find libbson via pkg-config");
    for path in bson.include_paths {
        builder = builder.clang_arg(format!("-I{}", path.display()))
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = builder
        // Otherwise it does not run :(
        .layout_tests(false)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = out_path.join("bindings.rs");
    println!("{:?}", out_path);
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}