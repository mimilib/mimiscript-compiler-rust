use crate::my_string;
use crate::py_arg::PyArg;
use crate::py_type::PyType;
use std::collections::HashMap;
use std::collections::*;
pub struct ArgList {
    py_arg_list: String,
    list: HashMap<String, PyArg>,
}

impl ArgList {
    pub fn to_string(&self) -> String {
        return self.py_arg_list.clone();
    }
    pub fn new(py_arg_list: &Option<String>) -> Option<ArgList> {
        let py_arg_list = match py_arg_list {
            Some(x) => x,
            None => return None,
        };
        let mut arg_list = ArgList {
            py_arg_list: py_arg_list.clone(),
            list: HashMap::new(),
        };
        let py_arg_list = py_arg_list.replace(" ", "");
        let py_arg_list: Vec<&str> = py_arg_list.split(",").collect();
        for (arg_define) in py_arg_list.iter() {
            let arg_name = match my_string::get_first_token(&arg_define.to_string(), ':') {
                Some(name) => name,
                None => return None,
            };
            let type_name = match my_string::get_last_token(&arg_define.to_string(), ':') {
                Some(name) => name,
                None => return None,
            };
            arg_list
                .list
                .entry(arg_name.clone())
                .or_insert(PyArg::new(&arg_name, &type_name));
        }
        return Some(arg_list);
    }
    pub fn to_c(&self) -> Option<String> {
        let mut arg_list_in_c = String::from("");
        for (i, (_, py_arg)) in self.list.iter().enumerate() {
            let arg_name = py_arg.name();
            let type_name_in_c = py_arg.c_type();
            arg_list_in_c.push_str(&type_name_in_c);
            arg_list_in_c.push_str(" ");
            arg_list_in_c.push_str(&arg_name);
            if i < self.list.len() - 1 {
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
        let arg_list = ArgList::new(&Some(String::from("arg1:str, arg2:int, arg3:FILE"))).unwrap();
        let arg_list_in_c = arg_list.to_c().unwrap();
        assert_eq! {arg_list_in_c,"int arg2, FILE * arg3, char * arg1"};
    }
}
