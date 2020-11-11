//! # binars
//! Binars system crate. dont use
#![allow(dead_code)]
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[test]
fn test_create(){
    unsafe {BinaryenModuleCreate()};
}