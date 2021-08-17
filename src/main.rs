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
    /* analyze each line of pikascript-api.py */
    for line in lines.iter() {
        compiler = Compiler::analyze_line(compiler, line.to_string());
    }
    /* write to compiler-info about the info */
    let mut compiler_info_file = File::create("dist/compiler-info.txt").unwrap();
    let compiler_info = format!("{:?}", compiler);
    compiler_info_file.write(compiler_info.as_bytes()).unwrap();
    /* make the api.c file for each python class */
    for (_, class_info) in compiler.class_list.iter() {
        let api_file_path = format!("dist/{}-api.c", class_info.this_class_name);
        let mut f = File::create(api_file_path).unwrap();
        f.write("/* ******************************** */\n".as_bytes())
            .unwrap();
        f.write("/* Warning! Don't modify this file! */\n".as_bytes())
            .unwrap();
        f.write("/* ******************************** */\n".as_bytes())
            .unwrap();
        f.write("#include <stdio.h>\n".as_bytes()).unwrap();
        f.write("#include <stdlib.h>\n".as_bytes()).unwrap();
        f.write("#include \"MimiObj.h\"\n".as_bytes()).unwrap();
        f.write(class_info.include().as_bytes()).unwrap();
        f.write("\n".as_bytes()).unwrap();
        f.write(class_info.method_api_fn().as_bytes()).unwrap();
        f.write(class_info.new_class_fn().as_bytes()).unwrap();
    }
}
