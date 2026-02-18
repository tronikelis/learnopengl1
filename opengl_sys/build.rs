use std::{env, path};

fn main() {
    println!("cargo:rustc-link-lib=OpenGL");

    bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("cant generate bindings")
        .write_to_file(path::PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("cant write bindings");
}
