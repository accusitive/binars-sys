extern crate bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    // let p = cmake::build("binaryen");
    let p = cmake::Config::new("binaryen")
    .define("BUILD_STATIC_LIB", "ON")
    .define("ENABLE_WERROR", "OFF")
    .build();
    let p = {
        cmake::Config::new("binaryen")
        .build()
    };

    println!("cargo:rustc-link-search=native={}/lib", p.display());
    println!("cargo:rustc-link-lib=static=binaryen");

    if let Some(cpp_stdlib) = get_cpp_stdlib() {
        println!("cargo:rustc-link-lib={}", cpp_stdlib);
    }
   // println!("cargo:rerun-if-changed={}", format!("{}/include/binaryen-c.h", p.to_str().unwrap()));
    
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate_comments(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
//https://github.com/pepyakin/binaryen-rs/blob/master/binaryen-sys/build.rs
fn get_cpp_stdlib() -> Option<String> {
    env::var("TARGET").ok().and_then(|target| {
        if target.contains("msvc") {
            None
        } else if target.contains("darwin") {
            Some("c++".to_string())
        } else if target.contains("freebsd") {
            Some("c++".to_string())
        } else if target.contains("musl") {
            Some("static=stdc++".to_string())
        } else {
            Some("stdc++".to_string())
        }
    })
}
