use crate::class_info::ClassInfo;
pub struct Compiler<'a> {
    pub class_list: Vec<ClassInfo>,
    pub class_now: &'a mut ClassInfo,
}

impl<'a> Compiler<'a> {
    fn analize_line(&mut self, line: String) -> Option<Self>{
        if line.starts_with("#") {
            return None;
        }
        if line.starts_with("class") {
            let mut class_now = match ClassInfo::new(&line) {
                Some(s) => s,
                None => return None,
            };
            self.class_list.push(class_now);
            self.class_now = self.class_list.last_mut().unwrap();
            return Some(*self);
        }
        if line.starts_with("    def ") {
            let line = line.strip_prefix("    ").unwrap().to_string();
            self.class_now.push_method(line);
            return Some(*self);
        }
        if line.starts_with("    ")
            && line.contains("(")
            && line.contains(")")
            && line.contains("=")
        {
            let line = line.strip_prefix("    ").unwrap().to_string();
            self.class_now.push_object(line);
            return Some(*self);
        }
        if line.starts_with("    ") && line.contains("(") && line.contains(")") {
            let line = line.strip_prefix("    ").unwrap().to_string();
            self.class_now.push_import(line);
            return Some(*self);
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze() {
        let mut compiler = Compiler{
            class_now: &mut ClassInfo::new(&String::from("class Root(BaseObj):")).unwrap(),
            class_list: vec![],
        };
        compiler.analize_line(String::from("class Test(SuperTest):"));
        compiler.analize_line(String::from("    def test()"));
    }
}
