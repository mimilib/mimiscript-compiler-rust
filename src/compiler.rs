use crate::class_info::ClassInfo;
use std::collections::HashMap;
pub struct Compiler {
    pub class_list: HashMap<String, ClassInfo>,
    pub class_now_name: Option<String>,
}

impl Compiler {
    fn analyze_line(mut compiler: Compiler, line: String) -> Option<Compiler> {
        if line.starts_with("#") {
            return Some(compiler);
        }
        if line.starts_with("class") {
            let class_now = match ClassInfo::new(&line) {
                Some(s) => s,
                None => return None,
            };
            let class_name = class_now.this_class_name.clone();
            compiler
                .class_list
                .entry(class_name.clone())
                .or_insert(class_now);
            compiler.class_now_name = Some(class_name.clone());
            return Some(compiler);
        }
        if line.starts_with("    def ") {
            let line = line.strip_prefix("    ").unwrap().to_string();
            let class_now = compiler
                .class_list
                .get_mut(&compiler.class_now_name.clone().unwrap())
                .unwrap();
            class_now.push_method(line);
            return Some(compiler);
        }
        if line.starts_with("    ")
            && line.contains("(")
            && line.contains(")")
            && line.contains("=")
        {
            let line = line.strip_prefix("    ").unwrap().to_string();
            let class_now = compiler
                .class_list
                .get_mut(&compiler.class_now_name.clone().unwrap())
                .unwrap();
            class_now.push_object(line);
            return Some(compiler);
        }
        if line.starts_with("    ") && line.contains("(") && line.contains(")") {
            let line = line.strip_prefix("    ").unwrap().to_string();
            let class_now = compiler
                .class_list
                .get_mut(&compiler.class_now_name.clone().unwrap())
                .unwrap();
            class_now.push_import(line);
            return Some(compiler);
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_analyze() {
        let compiler = Compiler {
            class_now_name: None,
            class_list: HashMap::new(),
        };
        let compiler =
            Compiler::analyze_line(compiler, String::from("class Test(SuperTest):")).unwrap();
        let compiler = Compiler::analyze_line(compiler, String::from("    def test()")).unwrap();
        let class_info = compiler.class_list.get("Test").unwrap();
        assert_eq!(class_info.this_class_name, "Test");
        assert_eq!(class_info.super_class_name, "SuperTest");
        let method_info = class_info.method_list.get("test").unwrap();
        assert_eq!(method_info.name, "test");
    }
}