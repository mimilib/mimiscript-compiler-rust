use crate::my_string;
pub struct MethodInfo {
    pub name: String,
    pub arg_list: Option<String>,
    pub return_type: Option<String>,
}

impl MethodInfo {
    pub fn from(define: String) -> Option<MethodInfo> {
        let define_without_prefix = define.strip_prefix("    def ").unwrap().to_string();
        let name = match my_string::get_first_token(&define_without_prefix, '(') {
            Some(token) => token,
            None => return None,
        };
        let arg_list = my_string::cut(&define_without_prefix, '(', ')');
        let return_type = my_string::cut(&define_without_prefix, '>', ':');
        let method_info = MethodInfo {
            name: name,
            arg_list: arg_list,
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
    }
}
