use std::{env, path::PathBuf};

fn main() {
    let host = env::var("HOST").unwrap();
    let target = env::var("TARGET").unwrap();

    let host_and_target_contain = |s| host.contains(s) && target.contains(s);

    let static_link = cfg!(feature = "static");

    if !static_link
        && !target.contains("msvc")
        && !(host_and_target_contain("apple")
            || host_and_target_contain("freebsd")
            || host_and_target_contain("dragonfly"))
    {
        let libarchive = pkg_config::Config::new()
            .cargo_metadata(true)
            .print_system_libs(false)
            .probe("libarchive");
        if libarchive.is_ok() {
            // libarchive is installed, so link it
            // and generate bindings.
            println!("cargo:rustc-link-lib=archive");
            return generate_bindings();
        }
    }

    if target.contains("msvc") && try_vcpkg() {
        return;
    }

    let mut cc = cc::Build::new();

    // Build libarchive from source and statically link it.
    if target.contains("msvc")
        || target.contains("pc-windows-gnu")
        || static_link
        || target != host
        || target.contains("musl")
    {
        // Build libarchive and generate the rust bindings using bindgen.
        build_source(&mut cc, &target);
        return generate_bindings();
    }

    panic!("unable to link libarchive");
}

fn build_source(cc: &mut cc::Build, target: &str) {}

#[cfg(not(target_env = "msvc"))]
fn try_vcpkg() -> bool {
    false
}

#[cfg(target_env = "msvc")]
fn try_vcpkg() -> bool {
    match vcpkg::Config::new().emit_includes(true).probe("libarchive") {
        Ok(_) => true,
        Err(e) => {
            println!("note, vcpkg did not find libarchive: {}", e);
            false
        }
    }
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
