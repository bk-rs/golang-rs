pub use golang_type_name;

use golang_type_name::TypeName;

pub mod array_type;
pub mod map_type;
pub mod pointer_type;
pub mod slice_type;

pub use self::array_type::{ArrayLength, ArrayType};
pub use self::map_type::MapType;
pub use self::pointer_type::PointerType;
pub use self::slice_type::SliceType;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Type {
    TypeName(TypeName),
    ArrayType(ArrayType),
    PointerType(PointerType),
    SliceType(SliceType),
    MapType(MapType),
}
