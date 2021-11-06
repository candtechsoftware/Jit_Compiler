pub mod statement;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::types::{BuiltInType, Type};
    #[test]
    fn it_works() {
        let t = Type::new("hello", BuiltInType::VOID);
        println!("{:?}", t);
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
