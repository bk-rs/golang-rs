use crate::Type;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SliceType {
    pub element: Box<Type>,
}
