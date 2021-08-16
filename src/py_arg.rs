use crate::my_string;
use crate::py_type::PyType;

pub struct PyArg {
    py_type: PyType,
    name: String,
}

impl PyArg {
    pub fn new(name: &String, type_name: &String) -> PyArg {
        let py_arg = PyArg{
            name: name.clone(),
            py_type: PyType::new(type_name),
        };
        return py_arg;
    }
    pub fn name(&self) -> String {
        return self.name.clone();
    }
    pub fn py_type(&self) -> String {
        return self.py_type.to_string();
    }
    pub fn c_type(&self) -> String {
        return self.py_type.to_c_type();
    }
    pub fn c_define(&self) -> String {
        let mut c_define = String::from("");
        c_define.push_str(&self.c_type());
        c_define.push_str(" ");
        c_define.push_str(&self.name());
        return c_define;
    }
}
