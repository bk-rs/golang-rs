use crate::Type;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ArrayType {
    pub length: ArrayLength,
    pub element: Box<Type>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ArrayLength {
    IntLiteral(usize),
    Other(String),
}
