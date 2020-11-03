extern crate bindgen;
use std::env;
use std::path::PathBuf;
fn get_generator()->  &'static str {
    if cfg!(windows){
        return "Visual Studio 15 2017"
    }else if cfg!(mac){
        return "Ninja"
    }
    else {
        "Ninja"
    }
}
fn main() {
    // let p = cmake::build("binaryen");
    let p = cmake::Config::new("binaryen").generator(get_generator()).build();
    std::fs::write("testing123", format!("{:?}", p)).unwrap();
    println!("{:?}", p);
    println!("cargo:rustc-link-search={}/lib", p.display());

    println!("cargo:rustc-link-lib=binaryen");
    println!("cargo:rerun-if-changed={}", format!("{}/include/binaryen-c.h", p.to_str().unwrap()));
    
    let bindings = bindgen::Builder::default()
        .header(format!("{}/include/binaryen-c.h", p.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
