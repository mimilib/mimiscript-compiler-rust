pub struct TypeList{
    contains: String,
}

impl TypeList{
    pub fn to_string(&self) -> String {
        return self.contains.clone();
    }
    pub fn new(contains: &Option<String>) -> Option<TypeList>{
        let contains = match contains{
            Some(contains) => contains,
            None => return None,
        };
        let type_list = TypeList {
            contains: contains.clone(),
        };
        return Some(type_list);
    }
}
