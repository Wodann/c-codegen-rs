use core::fmt;

pub mod enumeration;
mod integer;
mod pointer;
mod real;
mod scalar;

use integer::Integer;

use crate::macros::impl_froms;

pub use self::{
    integer::{IntegerKind, StrongInt},
    pointer::Pointer,
    real::Real,
    scalar::Scalar,
};

/// Source
///
/// https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Data-Types
#[derive(Clone)]
pub enum Type {
    Array(Box<Type>),
    Char,
    Enum(String),
    Integer(Integer),
    Pointer(Pointer),
    Real(Real),
    StrongInt(StrongInt),
    Struct(String),
    Union(String),
    Void,
}

impl_froms!(Type: Integer, Pointer, Real, StrongInt);

impl Type {
    pub const fn double() -> Self {
        Self::Real(Real::Double)
    }

    pub const fn int() -> Self {
        Self::Integer(Integer {
            kind: IntegerKind::Int,
            is_signed: true,
        })
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Array(base) => write!(f, "{}[]", base),
            Type::Char => write!(f, "char"),
            Type::Enum(name) => write!(f, "enum {}", name),
            Type::Integer(integer) => write!(f, "{integer}"),
            Type::Pointer(pointer) => write!(f, "{pointer}"),
            Type::Real(ty) => write!(f, "{}", ty),
            Type::StrongInt(ty) => write!(f, "{}", ty),
            Type::Struct(name) => write!(f, "struct {}", name),
            Type::Union(name) => write!(f, "union {}", name),
            Type::Void => write!(f, "void"),
        }
    }
}
