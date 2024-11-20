use core::fmt;

use super::{integer::Integer, IntegerKind, Pointer, Real, StrongInt};

#[derive(Clone)]
pub enum Scalar {
    Char,
    Integer(Integer),
    Pointer(Pointer),
    StrongInt(StrongInt),
    Real(Real),
    Void,
}

impl Scalar {
    pub const fn int() -> Self {
        Self::Integer(Integer {
            kind: IntegerKind::Int,
            is_signed: true,
        })
    }
}

impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Scalar::Char => write!(f, "char"),
            Scalar::Integer(integer) => write!(f, "{integer}"),
            Scalar::Pointer(pointer) => write!(f, "{pointer}"),
            Scalar::Real(ty) => write!(f, "{}", ty),
            Scalar::StrongInt(ty) => write!(f, "{}", ty),
            Scalar::Void => write!(f, "void"),
        }
    }
}
