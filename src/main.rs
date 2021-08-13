mod analize;
use analize::*;

fn main() {
    let class_info = ClassInfo::get_class_info_by_define(&String::from("test"));
    let class_info_print = class_info.print_info();
    println!("{}", class_info_print);
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 这个加法函数写得很差，本例中我们会使它失败。
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
