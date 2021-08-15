use crate::class_info::ClassInfo;
pub struct Compiler<'a> {
    pub class_list: Vec<ClassInfo>,
    pub class_now: &'a mut ClassInfo,
}

impl<'a> Compiler<'a> {
    fn analize_line(&'a mut self, line: String) -> bool {
        if line.starts_with("#") {
            return true;
        }
        if line.starts_with("class") {
            let mut class_now = match ClassInfo::new(&line) {
                Some(s) => s,
                None => return false,
            };
            self.class_list.push(class_now);
            self.class_now = self.class_list.last_mut().unwrap();
            return true;
        }
        if line.starts_with("    def ") {
            let line = line.strip_prefix("    ").unwrap().to_string();
            self.class_now.push_method(line);
            return true;
        }
        if line.starts_with("    ")
            && line.contains("(")
            && line.contains(")")
            && line.contains("=")
        {
            let line = line.strip_prefix("    ").unwrap().to_string();
            self.class_now.push_object(line);
            return true;
        }
        if line.starts_with("    ") && line.contains("(") && line.contains(")") {
            let line = line.strip_prefix("    ").unwrap().to_string();
            self.class_now.push_import(line);
            return true;
        }
        return false;
    }
}
