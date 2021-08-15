use crate::class_info::ClassInfo;
pub struct Compiler {
    pub class_now: &ClassInfo,
    pub class_list: Vec<ClassInfo>,
}

impl Compiler {
    fn analize_line(&mut self, line: String) -> bool {
        if line.starts_with("#") {
            return true;
        }
        if line.starts_with("class") {
            self.class_list.push(match ClassInfo::new(&line) {
                Some(s) => s,
                None => return false,
            })
        }
        if line.starts_with("    def ") {
            let line = line.strip_prefix("    ").unwrap().to_string();
            self.class_list.push(match ClassInfo::new(&line) {
                Some(s) => s,
                None => return false,
            })
        }
        if line.starts_with("    def ") {
            let line = line.strip_prefix("    ").unwrap().to_string();
            self.class_list.push(match ClassInfo::new(&line) {
                Some(s) => s,
                None => return false,
            })
        }

        return false;
    }
}
