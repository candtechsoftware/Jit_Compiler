#[derive(Debug)]
pub enum BuiltInType {
    VOID,
    INT8,
    UINT8,
    INT32,
    UINT32,
    DOUBLE,
    STRUCT,
}

#[derive(Debug)]
pub struct Type {
    name: String,
    item_type: BuiltInType,
    fields: Vec<Type>,
}

impl Type {
    pub fn new(name: &str, item_type: BuiltInType) -> Self {
        Self {
            name: String::from(name),
            item_type,
            fields: Vec::new(),
        }
    }
}
