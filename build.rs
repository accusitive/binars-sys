extern crate bindgen;
use std::env;
use std::path::PathBuf;
fn main () {
    let p = cmake::build("binaryen");
    std::fs::write("testing123", format!("{:?}", p)).unwrap();
    println!("{:?}", p);
    println!("cargo:rustc-link-search={}/out/lib", p.to_str().unwrap());

     println!("cargo:rustc-link-lib=binaryen");
 println!("cargo:rerun-if-changed=wrapper.h");
 let bindings = bindgen::Builder::default()
              // The input header we would like to generate
              // bindings for.
              .header("wrapper.h")
              // Tell cargo to invalidate the built crate whenever any of the
              // included header files changed.
              .parse_callbacks(Box::new(bindgen::CargoCallbacks))
              // Finish the builder and generate the bindings.
              .generate()
              // Unwrap the Result and panic on failure.
              .expect("Unable to generate bindings");
let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
     bindings
             .write_to_file(out_path.join("bindings.rs"))
             .expect("Couldn't write bindings!");
         let bindings = std::fs::read(out_path.join("bindings.rs")).unwrap();
}
