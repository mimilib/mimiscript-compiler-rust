use crate::my_string;

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
