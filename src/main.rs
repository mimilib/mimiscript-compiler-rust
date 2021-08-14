mod analyzer;
mod my_string;

fn main() {
    let class_info = analyzer::ClassInfo::get_class_info_by_define(String::from("class Test(SuperTest):"));
    let class_info_print = class_info.unwrap().print_info();
    println!("{}", class_info_print);
}
