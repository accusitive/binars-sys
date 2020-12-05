extern crate bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    // let p = cmake::build("binaryen");
    let p = {
        cmake::Config::new("binaryen")
        .build()
    };
    println!("cargo:rustc-link-search=dylib={}/lib", p.display());
    println!("cargo:rustc-link-lib=dylib=binaryen");
    std::fs::write("test", p.display().to_string()).unwrap();

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
