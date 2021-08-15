use crate::class_info::ClassInfo;
use std::collections::HashMap;
pub struct Compiler {
    pub class_list: HashMap<String, ClassInfo>,
    pub class_now_name: Option<String>,
}

// impl Compiler {
//     fn analize_line(mut compiler: Compiler, line: String) -> Option<Compiler> {
//         if line.starts_with("#") {
//             return Some(compiler);
//         }
//         if line.starts_with("class") {
//             let mut class_now = match ClassInfo::new(&line) {
//                 Some(s) => s,
//                 None => return None,
//             };
//             compiler.class_list.push(class_now);
//             compiler.class_now_name = match compiler.class_list.last() {
//                 Some(s) => Some(s.this_class_name.clone()),
//                 None => None,
//             };
//             return Some(compiler);
//         }
//         if line.starts_with("    def ") {
//             let line = line.strip_prefix("    ").unwrap().to_string();
//             let mut class_now: &mut ClassInfo;
//             for class_info in &compiler.class_list {
//                 if class_info.this_class_name == compiler.class_now_name.clone().unwrap() {
//                     class_now = &class_info;
//                 }
//             }
//             class_now.push_method(line);
//             return Some(compiler);
//         }
//         if line.starts_with("    ")
//             && line.contains("(")
//             && line.contains(")")
//             && line.contains("=")
//         {
//             let line = line.strip_prefix("    ").unwrap().to_string();
//             let class_now: &ClassInfo;
//             for class_info in &compiler.class_list {
//                 if class_info.this_class_name == compiler.class_now_name.unwrap() {
//                     class_now = &class_info;
//                 }
//             }
//             class_now.push_object(line);
//             return Some(compiler);
//         }
//         if line.starts_with("    ") && line.contains("(") && line.contains(")") {
//             let line = line.strip_prefix("    ").unwrap().to_string();
//             let class_now: &ClassInfo;
//             for class_info in &compiler.class_list {
//                 if class_info.this_class_name == compiler.class_now_name.unwrap() {
//                     class_now = &class_info;
//                 }
//             }
//             class_now.push_import(line);
//             return Some(compiler);
//         }
//         return None;
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_analyze() {
//         let mut compiler = Compiler {
//             class_now_name: &mut ClassInfo::new(&String::from("class Root(BaseObj):")).unwrap(),
//             class_list: vec![],
//         };
//         compiler.analize_line(String::from("class Test(SuperTest):"));
//         compiler.analize_line(String::from("    def test()"));
//     }
// }
