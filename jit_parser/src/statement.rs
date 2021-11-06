use crate::types::Type;

#[derive(Debug)]
pub enum StatementKind {
    VARIABLEDECLARATION,
    FUNCTIONCALL,
    LITERAL,
    OPERATORCALL,
}

#[derive(Debug)]
pub struct Statement<'a> {
    name: &'a str,
    statement_type: Type,
    params: Vec<Statement<'a>>,
    kind: StatementKind,
}

impl<'a> Statement<'a> {
    pub fn new(name: &'a str, statement_type: Type, kind: StatementKind) -> Self {
        Self {
            name,
            statement_type,
            params: Vec::new(),
            kind,
        }
    }

    pub fn add_param(mut self, param: Statement<'a>) {
        self.params.push(param);
    }
}
