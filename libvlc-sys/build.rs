use bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=vlc");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // For no_std
        .use_core()
        // Use libc
        .ctypes_prefix("libc")
        // Whitelist
        .whitelist_type(".*vlc.*")
        .whitelist_function(".*vlc.*")
        .whitelist_var(".*vlc.*")
        .whitelist_function("vsnprintf")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
