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
    use super::*;

    #[test]
    fn test_analize() {
        let class_info = ClassInfo::get_class_info_by_define(&String::from("test"));
        assert_eq!(class_info.this_calss_name, "test");
        assert_eq!(class_info.super_calss_name, "test");
    }
}
