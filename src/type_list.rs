use crate::my_string;
use crate::py_type::PyType;
use std::collections::*;
pub struct TypeList {
    contains: String,
}

impl TypeList {
    pub fn to_string(&self) -> String {
        return self.contains.clone();
    }
    pub fn new(contains: &Option<String>) -> Option<TypeList> {
        let contains = match contains {
            Some(contains) => contains,
            None => return None,
        };
        let type_list = TypeList {
            contains: contains.clone(),
        };
        return Some(type_list);
    }
    pub fn to_c(&self) -> Option<String> {
        let type_list = self.to_string();
        let type_list = type_list.replace(" ","");
        let type_list: Vec<&str> = type_list.split(",").collect();
        let mut type_list_in_c = String::from("");
        for (i, type_define) in type_list.iter().enumerate() {
            let arg_name = match my_string::get_first_token(&type_define.to_string(), ':') {
                Some(name) => name,
                None => return None,
            };
            let type_name = match my_string::get_last_token(&type_define.to_string(), ':') {
                Some(name) => name,
                None => return None,
            };
            let type_name_in_c = PyType::new(&type_name).to_c_type();
            type_list_in_c.push_str(&type_name_in_c);
            type_list_in_c.push_str(" ");
            type_list_in_c.push_str(&arg_name);
            if i < type_list.len() - 1 {
                type_list_in_c.push_str(", ");
            }
        }
        return Some(type_list_in_c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_list() {
        let type_list = TypeList::new(&Some(String::from("arg1:str, arg2:int"))).unwrap();
        let type_list_in_c = type_list.to_c().unwrap();
        assert_eq! {type_list_in_c,"char * arg1, int arg2"};
    }
}
