use crate::my_string;

pub struct ObjectInfo {
    pub name: String,
    pub class_name: String,
}

impl ObjectInfo {
    pub fn from(input_define: String) -> Option<ObjectInfo> {
        let define = input_define.replace(" ", "");
        let name = match my_string::get_first_token(&define, '=') {
            Some(token) => token,
            None => return None,
        };
        let class_name = match my_string::cut(&define, '=', '(') {
            Some(token) => token,
            None => return None,
        };
        return Some(ObjectInfo {
            name: name,
            class_name: class_name,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_info() {
        assert_eq!(
            ObjectInfo::from(String::from("test=Test()"))
                .unwrap()
                .class_name,
            String::from("Test")
        );
        assert_eq!(
            ObjectInfo::from(String::from("test=Test()")).unwrap().name,
            String::from("test")
        );
    }
}
