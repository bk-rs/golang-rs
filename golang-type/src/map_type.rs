use crate::Type;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MapType {
    pub key: Box<Type>,
    pub value: Box<Type>,
}
