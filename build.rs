extern crate bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    let p = cmake::build("binaryen");
    
    println!("{:?}", p);
   println!("cargo:rustc-link-search={}/lib", p.display());

    println!("cargo:rustc-link-lib=binaryen");
   // println!("cargo:rerun-if-changed={}", format!("{}/include/binaryen-c.h", p.to_str().unwrap()));
    
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
