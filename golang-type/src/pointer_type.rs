use crate::Type;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PointerType(Box<Type>);
