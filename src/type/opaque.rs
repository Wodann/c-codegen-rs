use crate::{macros::impl_froms, statement::Typedef};

use super::{
    Array, ConcreteType, Enum, Function, Integer, Pointer, Real, StrongInt, Struct, Union,
};

#[derive(Clone)]
pub enum OpaqueType {
    ConcreteType(ConcreteType),
    Function(Function),
}

impl_froms!(OpaqueType:
        ConcreteType(Array, Enum, Integer, box Pointer, Real, Struct, StrongInt, box Typedef, Union),
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
            OpaqueType::ConcreteType(ty) => ty.base_type(),
            ty => ty.clone(),
        }
    }

    /// Returns the innermost element type of the array, return type of a function, or otherwise the type itself.
    /// This is useful for determining the type of elements in a multi-dimensional array.
    ///
    /// # Examples
    ///
    /// - For `int`, it returns `int`.
    /// - For `int[3][4]`, it returns `int`.
    ///
    pub fn innermost_element_type(&self) -> ConcreteType {
        match self {
            Self::ConcreteType(ty) => ty.innermost_element_type(),
            Self::Function(function) => function.return_ty.clone(),
        }
    }
}
