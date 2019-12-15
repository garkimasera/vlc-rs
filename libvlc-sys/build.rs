use bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    let mut bindings = bindgen::Builder::default()
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
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    // Set header include paths
    let pkg_config_library = pkg_config::Config::new().probe("libvlc").unwrap();
    for include_path in &pkg_config_library.include_paths {
        bindings = bindings.clang_arg(format!("-I{}", include_path.display()));
    }

    let bindings = bindings.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
