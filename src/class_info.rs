use crate::import_info::ImportInfo;
use crate::method_info::MethodInfo;
use crate::my_string;
use crate::object_info::ObjectInfo;
pub struct ClassInfo {
    pub this_class_name: String,
    pub super_class_name: String,
    pub method_list: Vec<MethodInfo>,
    pub object_list: Vec<ObjectInfo>,
    pub import_list: Vec<ImportInfo>,
}

impl ClassInfo {
    pub fn new(define: String) -> Option<ClassInfo> {
        let define_without_prefix = define.strip_prefix("class ").unwrap().to_string();
        let super_class_name = match my_string::cut(&define_without_prefix, '(', ')') {
            Some(s) => s,
            None => return None,
        };
        let this_calss_name = match my_string::get_first_token(&define_without_prefix, '(') {
            Some(s) => s,
            None => return None,
        };
        let new_class_info = ClassInfo {
            this_class_name: this_calss_name,
            super_class_name: super_class_name,
            method_list: vec![],
            object_list: vec![],
            import_list: vec![],
        };
        return Some(new_class_info);
    }
    pub fn push_method(&mut self, method_define: String) {
        self.method_list.push(
            match MethodInfo::new(&self.this_class_name, method_define) {
                Some(method) => method,
                None => return,
            },
        );
    }
    pub fn push_import(&mut self, import_define: String) {
        self.import_list.push(
            match ImportInfo::new(&self.this_class_name, import_define) {
                Some(import) => import,
                None => return,
            },
        );
    }
    pub fn push_object(&mut self, object_define: String) {
        self.object_list.push(
            match ObjectInfo::new(&self.this_class_name, object_define) {
                Some(object) => object,
                None => return,
            },
        );
    }

    pub fn print_info(&self) -> String {
        return format!(
            "[info] this class name: {}, super class name: {}",
            self.this_class_name, self.super_class_name
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analize() {
        assert_eq!(
            ClassInfo::new(String::from("class Test(SuperTest):"))
                .unwrap()
                .this_class_name,
            "Test"
        );
        assert_eq!(
            ClassInfo::new(String::from("class Test(SuperTest):"))
                .unwrap()
                .super_class_name,
            "SuperTest"
        );
    }
    #[test]
    fn test_push_method() {
        let mut class_info = ClassInfo::new(String::from("class Test(SuperTest):")).unwrap();
        class_info.push_method(String::from("def test(data: str)-> str:"));
        assert_eq!(class_info.method_list[0].class_name, "Test");
        assert_eq!(class_info.method_list[0].name, "test");
        assert_eq!(
            class_info.method_list[0].return_type.clone().unwrap(),
            "str"
        );
        assert_eq!(
            class_info.method_list[0].type_list.clone().unwrap(),
            "data:str"
        );
    }
    #[test]
    fn test_push_object() {
        let mut class_info = ClassInfo::new(String::from("class Test(SuperTest):")).unwrap();
        class_info.push_object(String::from("testObj = TestObj()"));
        assert_eq!(class_info.object_list[0].class_name, "Test");
        assert_eq!(class_info.object_list[0].name, "testObj");
        assert_eq!(class_info.object_list[0].import_class_name, "TestObj");
        assert_eq!(class_info.object_list[0].name, "testObj");
    }
    #[test]
    fn test_push_import() {
        let mut class_info = ClassInfo::new(String::from("class Test(SuperTest):")).unwrap();
        class_info.push_import(String::from("TestObj()"));
        assert_eq!(class_info.import_list[0].class_name, "Test");
        assert_eq!(class_info.import_list[0].import_class_name, "TestObj");
    }
}
