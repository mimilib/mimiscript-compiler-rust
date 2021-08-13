mod analize;
use analize::*;

fn main() {
    let class_info = ClassInfo::get_class_info_by_define(&String::from("test"));
    let class_info_print = class_info.print_info();
    println!("{}", class_info_print);
}

