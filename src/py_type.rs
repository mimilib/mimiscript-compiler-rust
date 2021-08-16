pub struct PyType {
    type_name: String,
}

impl PyType {
    pub fn to_c_type(self) -> String {
        if self.type_name == "int" {
            return "int".to_string();
        }
        if self.type_name == "float" {
            return "float".to_string();
        }
        if self.type_name == "pointer" {
            return "void *".to_string();
        }
        if self.type_name == "str" {
            return "char *".to_string();
        }
        if self.type_name == "" {
            return "void".to_string();
        }
        return format!("{} *", self.type_name);
    }
    pub fn to_string(&self) -> String {
        return self.type_name.clone();
    }
    pub fn new(type_name: &String) -> PyType {
        return PyType {
            type_name: type_name.clone(),
        };
    }
    fn get_return_fun_name(&self) -> String {
        if self.type_name == "int" {
            return "method_returnInt".to_string();
        }
        if self.type_name == "float" {
            return "method_returnFloat".to_string();
        }
        if self.type_name == "pointer" {
            return "method_returnPtr".to_string();
        }
        if self.type_name == "str" {
            return "method_returnStr".to_string();
        }
        return "method_returnPtr".to_string();
    }
}
