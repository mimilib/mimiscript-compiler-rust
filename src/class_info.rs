use crate::import_info::ImportInfo;
use crate::method_info::MethodInfo;
use crate::my_string;
use crate::object_info::ObjectInfo;
use std::collections::BTreeMap;
#[derive(Debug)]
pub struct ClassInfo {
    pub this_class_name: String,
    pub super_class_name: String,
    pub method_list: BTreeMap<String, MethodInfo>,
    pub object_list: BTreeMap<String, ObjectInfo>,
    pub import_list: BTreeMap<String, ImportInfo>,
}

impl ClassInfo {
    pub fn new(define: &String) -> Option<ClassInfo> {
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
            method_list: BTreeMap::new(),
            object_list: BTreeMap::new(),
            import_list: BTreeMap::new(),
        };
        return Some(new_class_info);
    }
    pub fn push_method(&mut self, method_define: String) {
        let method_info = match MethodInfo::new(&self.this_class_name, method_define) {
            Some(method) => method,
            None => return,
        };
        self.method_list
            .entry(method_info.name.clone())
            .or_insert(method_info);
    }
    pub fn push_import(&mut self, import_define: String) {
        let import_info = match ImportInfo::new(&self.this_class_name, import_define) {
            Some(import) => import,
            None => return,
        };
        self.import_list
            .entry(import_info.import_class_name.clone())
            .or_insert(import_info);
    }
    pub fn push_object(&mut self, object_define: String) {
        let object_info = match ObjectInfo::new(&self.this_class_name, object_define) {
            Some(object) => object,
            None => return,
        };
        self.object_list
            .entry(object_info.name.clone())
            .or_insert(object_info);
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
            ClassInfo::new(&String::from("class Test(SuperTest):"))
                .unwrap()
                .this_class_name,
            "Test"
        );
        assert_eq!(
            ClassInfo::new(&String::from("class Test(SuperTest):"))
                .unwrap()
                .super_class_name,
            "SuperTest"
        );
    }
    #[test]
    fn test_push_method() {
        let mut class_info = ClassInfo::new(&String::from("class Test(SuperTest):")).unwrap();
        class_info.push_method(String::from("def test(data: str)-> str:"));
        assert_eq!(
            class_info.method_list.get("test").unwrap().class_name,
            "Test"
        );
        assert_eq!(class_info.method_list.get("test").unwrap().name, "test");
        assert_eq!(
            class_info
                .method_list
                .get("test")
                .as_ref()
                .unwrap()
                .return_type
                .as_ref()
                .unwrap()
                .to_string(),
            "str"
        );
        assert_eq!(
            class_info
                .method_list
                .get("test")
                .as_ref()
                .unwrap()
                .arg_list
                .as_ref()
                .unwrap()
                .to_string(),
            "data:str"
        );
    }
    #[test]
    fn test_push_object() {
        let mut class_info = ClassInfo::new(&String::from("class Test(SuperTest):")).unwrap();
        class_info.push_object(String::from("testObj = TestObj()"));
        assert_eq!(
            class_info.object_list.get("testObj").unwrap().class_name,
            "Test"
        );
        assert_eq!(
            class_info.object_list.get("testObj").unwrap().name,
            "testObj"
        );
        assert_eq!(
            class_info
                .object_list
                .get("testObj")
                .unwrap()
                .import_class_name,
            "TestObj"
        );
        assert_eq!(
            class_info.object_list.get("testObj").unwrap().name,
            "testObj"
        );
    }
    #[test]
    fn test_push_import() {
        let mut class_info = ClassInfo::new(&String::from("class Test(SuperTest):")).unwrap();
        class_info.push_import(String::from("TestObj()"));
        assert_eq!(
            class_info.import_list.get("TestObj").unwrap().class_name,
            "Test"
        );
        assert_eq!(
            class_info
                .import_list
                .get("TestObj")
                .unwrap()
                .import_class_name,
            "TestObj"
        );
    }
}
