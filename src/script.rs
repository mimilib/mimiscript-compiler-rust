use crate::class_info::ClassInfo;
use crate::my_string;
#[derive(Debug)]
pub struct Script {
    content: String,
}

impl Script {
    pub fn new(content: &String) -> Script {
        let script = Script {
            content: content.clone(),
        };
        return script;
    }
    pub fn assert(class_info: &ClassInfo, content: &String) -> bool {
        let cmd = my_string::cut(content, '=', '(').unwrap();
        let cmd = cmd.replace(" ", "");
        let called_first_object = match my_string::get_first_token(&cmd, '.') {
            Some(token) => token,
            None => cmd,
        };
        for (_, obj_info) in class_info.object_list.iter() {
            let obj_name = obj_info.name.clone();
            if called_first_object == obj_name {
                return true;
            }
        }
        return false;
    }
}
