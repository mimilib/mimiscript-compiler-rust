use crate::my_string;

pub struct MethodInfo {
    pub class_name: String,
    pub name: String,
    pub type_list: Option<String>,
    pub return_type: Option<String>,
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
        let return_type = my_string::cut(&define, '>', ':');
        let method_info = MethodInfo {
            name: name,
            type_list: type_list,
            return_type: return_type,
            class_name: class_name.clone(),
        };
        return Some(method_info);
    }
    pub fn get_define(self) -> String {
        let return_token = match self.return_type {
            Some(s) => format!("->{}", s),
            None => String::from(""),
        };
        let type_list = match self.type_list {
            Some(t) => t,
            None => String::from(""),
        };
        let define = format!(
            "    class_defineMethod(self, \"{}({}){}\", {}_{}Method);\n",
            self.name, type_list, return_token, self.class_name, self.name
        );
        return define;
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
            .unwrap(),
            String::from("str")
        );
        assert_eq!(
            MethodInfo::new(
                &String::from("Test"),
                String::from("def test(test:str)->str:")
            )
            .unwrap()
            .type_list
            .unwrap(),
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
            .unwrap(),
            String::from("str")
        );
        assert_eq!(
            MethodInfo::new(
                &String::from("Test"),
                String::from("def test(test: str) ->str:")
            )
            .unwrap()
            .type_list
            .unwrap(),
            String::from("test:str")
        );
        assert_eq!(
            MethodInfo::new(&String::from("Test"), String::from("def test() ->str:"))
                .unwrap()
                .type_list,
            None
        );
        assert_eq!(
            MethodInfo::new(
                &String::from("Test"),
                String::from("def test(test: str, test2: int) ->str:")
            )
            .unwrap()
            .type_list
            .unwrap(),
            String::from("test:str,test2:int")
        );
        assert_eq!(
            MethodInfo::new(
                &String::from("Test"),
                String::from("def test(test: str, test2: int):")
            )
            .unwrap()
            .return_type,
            None
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
        assert_eq!(define, String::from("    class_defineMethod(self, \"test(test:str,test2:int)\", Test_testMethod);\n"));
    }
    #[test]
    fn test_get_define_no_return_no_type_list() {
        let method_info = MethodInfo::new(
            &String::from("Test"),
            String::from("def test():"),
        );
        let define = method_info.unwrap().get_define();
        assert_eq!(define, String::from("    class_defineMethod(self, \"test()\", Test_testMethod);\n"));
    }
}
