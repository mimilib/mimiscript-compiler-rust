use crate::my_string;
#[derive(Debug)]
pub struct ImportInfo {
    pub class_name: String,
    pub import_class_name: String,
}

impl ImportInfo {
    pub fn new(class_name: &String, input_define: String) -> Option<ImportInfo> {
        let define = input_define.replace(" ", "");
        let import_class_name = match my_string::get_first_token(&define, '(') {
            Some(token) => token,
            None => return None,
        };
        return Some(ImportInfo {
            class_name: class_name.clone(),
            import_class_name: import_class_name,
        });
    }
    pub fn import_fn(&self) -> String {
        return format!(
            "    obj_import(self, \"{}\", New_{});\n",
            self.import_class_name, self.import_class_name
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_info() {
        assert_eq!(
            ImportInfo::new(&String::from("Test"), String::from("ImportTest()"))
                .unwrap()
                .import_class_name,
            String::from("ImportTest")
        );
        assert_eq!(
            ImportInfo::new(&String::from("Test"), String::from("ImportTest()"))
                .unwrap()
                .class_name,
            String::from("Test")
        );
    }
}
