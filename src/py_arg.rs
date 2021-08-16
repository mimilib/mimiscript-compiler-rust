use crate::my_string;
use crate::py_type::PyType;

pub struct PyArg {
    py_type: PyType,
    name: String,
}

impl PyArg {
    pub fn name(&self)->String{
        return self.name.clone();
    }
    pub fn py_type(&self)->String{
        return self.py_type.to_string();
    }
    pub fn c_type(&self)->String{
        return self.py_type.to_c_type();
    }
}
