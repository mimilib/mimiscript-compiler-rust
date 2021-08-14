use crate::my_string;
pub struct ClassInfo {
    pub this_calss_name: String,
    pub super_calss_name: String,
}

impl ClassInfo {
    pub fn get_class_info_by_define(class_define: String) -> Option<ClassInfo> {
        let mut new_class_info = ClassInfo {
            this_calss_name: String::from("(none)"),
            super_calss_name: String::from("(none)"),
        };
        let class_define_without_prefix = class_define.strip_prefix("class ").unwrap().to_string();
        new_class_info.super_calss_name =
            match my_string::cut(&class_define_without_prefix, '(', ')') {
                Some(s) => s,
                None => return None,
            };
        new_class_info.this_calss_name =
            match my_string::get_first_token(&class_define_without_prefix, '(') {
                Some(s) => s,
                None => return None,
            };
        return Some(new_class_info);
    }
    pub fn print_info(&self) -> String {
        return format!(
            "[info] this class name: {}, super class name: {}",
            self.this_calss_name, self.super_calss_name
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analize() {
        assert_eq!(
            ClassInfo::get_class_info_by_define(String::from("class Test(SuperTest):"))
                .unwrap()
                .this_calss_name,
            "Test"
        );
        assert_eq!(
            ClassInfo::get_class_info_by_define(String::from("class Test(SuperTest):"))
                .unwrap()
                .super_calss_name,
            "SuperTest"
        );
    }
}
