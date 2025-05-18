use crate::{macros::impl_froms, statement::Typedef};

use super::{
    enumeration::Declaration as Enum, structure::Definition as Struct, union::Definition as Union,
    Array, Function, IncompleteType, Integer, Pointer, Real, StrongInt,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OpaqueType {
    IncompleteType(IncompleteType),
    Function(Function),
}

impl_froms!(OpaqueType:
        IncompleteType(Array, Enum, Integer, box Pointer, Real, Struct, StrongInt, box Typedef, Union),
        Function
);

impl OpaqueType {
    /// Returns the fundamental type of the instance, after stripping away all type constructors (like pointers and arrays).
    ///
    /// # Examples
    ///
    /// - For `int[3][4]`, it returns `int`.
    /// - For `void (*)(int, int)`, it returns `void (int, int)`.
    pub fn base_type(&self) -> OpaqueType {
        match self {
            OpaqueType::IncompleteType(ty) => ty.base_type(),
            ty => ty.clone(),
        }
    }
}
