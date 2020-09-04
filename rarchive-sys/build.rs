use std::{env, path::PathBuf};

fn main() {
    let static_link = cfg!(feature = "static");

    pkg_config::Config::new()
        .statik(static_link)
        .cargo_metadata(true)
        .print_system_libs(false)
        .probe("libarchive")
        .expect("failed to find libarchive");

    generate_bindings();
}

fn generate_bindings() {
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .whitelist_function("archive_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
