use crate::my_string;
use crate::py_type::PyType;
use crate::py_arg::PyArg;
use std::collections::HashMap;
use std::collections::*;
pub struct ArgList {
    contains: String,
    arg_list: HashMap<String, PyArg>,
}

impl ArgList {
    pub fn to_string(&self) -> String {
        return self.contains.clone();
    }
    pub fn new(contains: &Option<String>) -> Option<ArgList> {
        let contains = match contains {
            Some(contains) => contains,
            None => return None,
        };
        let arg_list = ArgList {
            contains: contains.clone(),
        };
        return Some(arg_list);
    }
    pub fn to_c(&self) -> Option<String> {
        let arg_list = self.to_string();
        let arg_list = arg_list.replace(" ","");
        let arg_list: Vec<&str> = arg_list.split(",").collect();
        let mut arg_list_in_c = String::from("");
        for (i, type_define) in arg_list.iter().enumerate() {
            let arg_name = match my_string::get_first_token(&type_define.to_string(), ':') {
                Some(name) => name,
                None => return None,
            };
            let type_name = match my_string::get_last_token(&type_define.to_string(), ':') {
                Some(name) => name,
                None => return None,
            };
            let type_name_in_c = PyType::new(&type_name).to_c_type();
            arg_list_in_c.push_str(&type_name_in_c);
            arg_list_in_c.push_str(" ");
            arg_list_in_c.push_str(&arg_name);
            if i < arg_list.len() - 1 {
                arg_list_in_c.push_str(", ");
            }
        }
        return Some(arg_list_in_c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arg_list() {
        let arg_list = ArgList::new(&Some(String::from("arg1:str, arg2:int"))).unwrap();
        let arg_list_in_c = arg_list.to_c().unwrap();
        assert_eq! {arg_list_in_c,"char * arg1, int arg2"};
    }
}
