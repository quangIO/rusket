use std::{env, path::PathBuf};

use bindgen;
use cc;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .layout_tests(false)
        .generate()
        .expect("Unable to genarate bindings from wrapper.h");

    let pwd = env::current_dir().unwrap();
    let lib_path = PathBuf::from(pwd.join("racket"));

    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = PathBuf::from(pwd.join("src"));
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings to file");

    if std::env::var("TARGET").unwrap().contains("-apple") {
        println!("cargo:rustc-link-lib=framework=Foundation");
    }
    println!("cargo:rustc-link-search={}", lib_path.display());
    println!("cargo:rustc-link-lib=racketcs");
    println!("cargo:rustc-link-lib=iconv");
    println!("cargo:rustc-link-lib=ncurses");

    cc::Build::new()
        .file(lib_path.join("util.c"))
        .compile("racketcs-util");
}
