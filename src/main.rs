mod class_info;
mod method_info;
mod object_info;
mod import_info;
mod my_string;
use class_info::*;
use method_info::*;
use object_info::*;
use import_info::*;

fn main() {
    MethodInfo::from(String::from("    def test(test:str)->str:"))
        .unwrap()
        .name;
}
