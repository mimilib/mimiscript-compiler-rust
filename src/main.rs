mod arg_list;
mod class_info;
mod compiler;
mod import_info;
mod method_info;
mod my_string;
mod object_info;
mod py_arg;
mod py_type;
use class_info::*;
use compiler::*;
use import_info::*;
use method_info::*;
use object_info::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("pikascript-api.py").unwrap();
    let mut picascript_api = String::new();
    file.read_to_string(&mut picascript_api).unwrap();
    let lines: Vec<&str> = picascript_api.split('\n').collect();
    for (i, line) in lines.iter().enumerate() {
        println!("{:3}|{}", i + 1, line);
    }
}
