use crate::my_string;
#[derive(Debug)]
pub struct ObjectInfo {
    pub class_name: String,
    pub name: String,
    pub import_class_name: String,
}

impl ObjectInfo {
    pub fn new(class_name: &String, input_define: String) -> Option<ObjectInfo> {
        let define = input_define.replace(" ", "");
        let name = match my_string::get_first_token(&define, '=') {
            Some(token) => token,
            None => return None,
        };
        let import_class_name = match my_string::cut(&define, '=', '(') {
            Some(token) => token,
            None => return None,
        };
        return Some(ObjectInfo {
            class_name: class_name.clone(),
            name: name,
            import_class_name: import_class_name,
        });
    }
    pub fn new_object_fun(&self) -> String {
        let mut new_object_fun = String::new();
        let import_fun = format!(
            "    obj_import(self, \"{}\", New_{});\n",
            self.import_class_name, self.import_class_name
        );
        new_object_fun.push_str(&import_fun);
        let new_fun = format!(
            "    obj_newObj(self, \"{}\", \"{}\");\n",
            self.name, self.import_class_name
        );
        new_object_fun.push_str(&new_fun);
        return new_object_fun;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_info() {
        assert_eq!(
            ObjectInfo::new(&String::from("Test"), String::from("test=ImportTest()"))
                .unwrap()
                .import_class_name,
            String::from("ImportTest")
        );
        assert_eq!(
            ObjectInfo::new(&String::from("Test"), String::from("test=ImportTest()"))
                .unwrap()
                .name,
            String::from("test")
        );
        assert_eq!(
            ObjectInfo::new(&String::from("Test"), String::from("test=ImportTest()"))
                .unwrap()
                .class_name,
            String::from("Test")
        );
    }
}
