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
