pub struct ClassInfo {
    pub this_calss_name: String,
    pub super_calss_name: String,
}

impl ClassInfo {
    pub fn analize_class_define(mut self, class_define: &String) -> Self {
        self.this_calss_name = class_define.to_string();
        self.super_calss_name = class_define.to_string();
        return self;
    }
    pub fn print(&self) {
        println!("{}{}", self.super_calss_name, self.this_calss_name);
    }
}

#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_analize() {
        let class_info_void = ClassInfo {
            this_calss_name: String::from("none"),
            super_calss_name: String::from("none"),
        };
        let class_info = class_info_void.analize_class_define(&String::from("test"));
        println!("test print");
        class_info.print();
    }
}
