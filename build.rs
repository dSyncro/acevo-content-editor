use std::path::PathBuf;

use bindgen::callbacks::ParseCallbacks;

fn main() {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/bindings/source/wrapper.hpp")
        .enable_cxx_namespaces()
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(CustomCallbacks::default()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("src/bindings/bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}

#[derive(Debug, Default)]
struct CustomCallbacks {}

impl ParseCallbacks for CustomCallbacks {
    fn add_derives(&self, _info: &bindgen::callbacks::DeriveInfo<'_>) -> Vec<String> {
        vec!["::bytemuck::Pod".into(), "::bytemuck::Zeroable".into()]
    }
}