mod arg_list;
mod class_info;
mod compiler;
mod import_info;
mod method_info;
mod my_string;
mod object_info;
mod py_arg;
mod py_type;
use compiler::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("pikascript-api.py").unwrap();
    let mut picascript_api = String::new();
    file.read_to_string(&mut picascript_api).unwrap();
    let lines: Vec<&str> = picascript_api.split('\n').collect();
    let mut compiler = Compiler::new();
    for (i, line) in lines.iter().enumerate() {
        println!("{:3}|{}", i + 1, line);
        compiler = Compiler::analyze_line(compiler, line.to_string());
    }
    let mut compiler_info_file = File::create("dist/compiler-info.txt").unwrap();
    let compiler_info = format!("{:?}", compiler);
    compiler_info_file.write(compiler_info.as_bytes()).unwrap();
}
