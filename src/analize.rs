pub struct ClassInfo {
    pub this_calss_name: String,
    pub super_calss_name: String,
}

impl ClassInfo {
    pub fn get_class_info_by_define(class_define: &String) -> ClassInfo{
        let mut new_class_info = ClassInfo {
            this_calss_name: String::from("none"),
            super_calss_name: String::from("none"),
        };
        new_class_info.this_calss_name = class_define.to_string();
        new_class_info.super_calss_name = class_define.to_string();
        return new_class_info;
    }
    pub fn print_info(&self) -> String {
        return format!(
            "[info] this class name: {}, super class name: {}",
            self.super_calss_name, self.this_calss_name
        );
    }
}

#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_analize() {
        let class_info = ClassInfo::get_class_info_by_define(&String::from("test"));
        let class_info_print = class_info.print_info();
        println!("{}", class_info_print);
    }
}
