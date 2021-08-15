use crate::my_string;

pub struct MethodInfo {
    pub name: String,
    pub type_list: Option<String>,
    pub return_type: Option<String>,
}

impl MethodInfo {
    pub fn from(input_define: String) -> Option<MethodInfo> {
        let define = match input_define.strip_prefix("    def ") {
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
        };
        return Some(method_info);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analize() {
        assert_eq!(
            MethodInfo::from(String::from("    def test(test:str)->str:"))
                .unwrap()
                .name,
            String::from("test")
        );
        assert_eq!(
            MethodInfo::from(String::from("    def test(test:str)->str:"))
                .unwrap()
                .return_type
                .unwrap(),
            String::from("str")
        );
        assert_eq!(
            MethodInfo::from(String::from("    def test(test:str)->str:"))
                .unwrap()
                .type_list
                .unwrap(),
            String::from("test:str")
        );
        assert_eq!(
            MethodInfo::from(String::from("    def test(test: str) ->str:"))
                .unwrap()
                .name,
            String::from("test")
        );
        assert_eq!(
            MethodInfo::from(String::from("    def test(test: str) ->str:"))
                .unwrap()
                .return_type
                .unwrap(),
            String::from("str")
        );
        assert_eq!(
            MethodInfo::from(String::from("    def test(test: str) ->str:"))
                .unwrap()
                .type_list
                .unwrap(),
            String::from("test:str")
        );
        assert_eq!(
            MethodInfo::from(String::from("    def test() ->str:"))
                .unwrap()
                .type_list,
            None
        );
        assert_eq!(
            MethodInfo::from(String::from("    def test(test: str, test2: int) ->str:"))
                .unwrap()
                .type_list
                .unwrap(),
            String::from("test:str,test2:int")
        );
        assert_eq!(
            MethodInfo::from(String::from("    def test(test: str, test2: int)"))
                .unwrap()
                .return_type,
            None
        );
    }
}
