use crate::my_string;
use crate::py_type::PyType;
use crate::type_list::TypeList;

pub struct MethodInfo {
    pub class_name: String,
    pub name: String,
    pub type_list: Option<TypeList>,
    pub return_type: Option<PyType>,
}

impl MethodInfo {
    pub fn new(class_name: &String, input_define: String) -> Option<MethodInfo> {
        let define = match input_define.strip_prefix("def ") {
            Some(define) => define.to_string(),
            None => return None,
        };
        let define = define.replace(" ", "");
        let name = match my_string::get_first_token(&define, '(') {
            Some(token) => token,
            None => return None,
        };
        let type_list = my_string::cut(&define, '(', ')');
        let return_type = match my_string::cut(&define, '>', ':') {
            Some(py_type) => Some(PyType::new(&py_type)),
            None => None,
        };
        let method_info = MethodInfo {
            name: name,
            type_list: TypeList::new(&type_list),
            return_type: return_type,
            class_name: class_name.clone(),
        };
        return Some(method_info);
    }
    pub fn get_define(&self) -> String {
        let return_token = match &self.return_type {
            Some(s) => format!("->{}", s.to_string()),
            None => String::from(""),
        };
        let type_list = match &self.type_list {
            Some(t) => t.to_string(),
            None => String::from(""),
        };
        let define = format!(
            "    class_defineMethod(self, \"{}({}){}\", {}_{}Method);\n",
            self.name, type_list, return_token, self.class_name, self.name
        );
        return define;
    }
    pub fn get_method_fun_name(&self) -> String {
        return format!(
            "void {}_{}Method(MimiObj *self, Args *args){{\n",
            self.class_name, self.name
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analize() {
        assert_eq!(
            MethodInfo::new(
                &String::from("Test"),
                String::from("def test(test:str)->str:")
            )
            .unwrap()
            .name,
            String::from("test")
        );
        assert_eq!(
            MethodInfo::new(
                &String::from("Test"),
                String::from("def test(test:str)->str:")
            )
            .unwrap()
            .return_type
            .unwrap()
            .to_string(),
            String::from("str")
        );
        assert_eq!(
            MethodInfo::new(
                &String::from("Test"),
                String::from("def test(test:str)->str:")
            )
            .unwrap()
            .type_list
            .unwrap()
            .to_string(),
            String::from("test:str")
        );
        assert_eq!(
            MethodInfo::new(
                &String::from("Test"),
                String::from("def test(test: str) ->str:")
            )
            .unwrap()
            .name,
            String::from("test")
        );
        assert_eq!(
            MethodInfo::new(
                &String::from("Test"),
                String::from("def test(test: str) ->str:")
            )
            .unwrap()
            .return_type
            .unwrap()
            .to_string(),
            String::from("str")
        );
        assert_eq!(
            MethodInfo::new(
                &String::from("Test"),
                String::from("def test(test: str) ->str:")
            )
            .unwrap()
            .type_list
            .unwrap()
            .to_string(),
            String::from("test:str")
        );
        assert_eq!(
            MethodInfo::new(
                &String::from("Test"),
                String::from("def test(test: str, test2: int) ->str:")
            )
            .unwrap()
            .type_list
            .unwrap()
            .to_string(),
            String::from("test:str,test2:int")
        );
    }
    #[test]
    fn test_get_define() {
        let method_info = MethodInfo::new(
            &String::from("Test"),
            String::from("def test(test:str, test2:int)->str:"),
        );
        let define = method_info.unwrap().get_define();
        assert_eq!(define, String::from("    class_defineMethod(self, \"test(test:str,test2:int)->str\", Test_testMethod);\n"));
    }
    #[test]
    fn test_get_define_no_return() {
        let method_info = MethodInfo::new(
            &String::from("Test"),
            String::from("def test(test:str, test2:int):"),
        );
        let define = method_info.unwrap().get_define();
        assert_eq!(
            define,
            String::from(
                "    class_defineMethod(self, \"test(test:str,test2:int)\", Test_testMethod);\n"
            )
        );
    }
    #[test]
    fn test_get_define_no_return_no_type_list() {
        let method_info =
            MethodInfo::new(&String::from("Test"), String::from("def test():")).unwrap();
        let define = method_info.get_define();
        let method_fun_name = method_info.get_method_fun_name();
        assert_eq!(
            define,
            String::from("    class_defineMethod(self, \"test()\", Test_testMethod);\n")
        );
        assert_eq!(
            method_fun_name,
            String::from("void Test_testMethod(MimiObj *self, Args *args){\n")
        );
    }
}
