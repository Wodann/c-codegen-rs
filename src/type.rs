use core::fmt;

mod r#integer;
mod real;

pub use self::{
    r#integer::{IntegerType, StrongInt},
    real::RealType,
};

/// Source
///
/// https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Data-Types
#[derive(Clone, Debug)]
pub enum CType {
    Array(Box<CType>),
    Char,
    Custom(String),
    Enum(String),
    Integer { ty: IntegerType, signed: bool },
    Pointer(Box<CType>, bool),
    Real(RealType),
    StrongInt(StrongInt),
    Struct(String),
    Union(String),
    Void,
}

impl CType {
    pub const fn double() -> Self {
        Self::Real(RealType::Double)
    }

    pub const fn int() -> Self {
        Self::Integer {
            ty: IntegerType::Int,
            signed: true,
        }
    }
}

impl fmt::Display for CType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CType::Array(base) => write!(f, "{}[]", base),
            CType::Char => {
                write!(f, "signed char")
            }
            CType::Custom(name) => write!(f, "{}", name),
            CType::Enum(name) => write!(f, "enum {}", name),
            CType::Integer { ty, signed } => {
                if *signed {
                    write!(f, "{}", ty)
                } else {
                    write!(f, "unsigned {}", ty)
                }
            }
            CType::Pointer(base, is_const) => {
                if *is_const {
                    write!(f, "const {}*", base)
                } else {
                    write!(f, "{}*", base)
                }
            }
            CType::Real(ty) => write!(f, "{}", ty),
            CType::StrongInt(ty) => write!(f, "{}", ty),
            CType::Struct(name) => write!(f, "struct {}", name),
            CType::Union(name) => write!(f, "union {}", name),
            CType::Void => write!(f, "void"),
        }
    }
}
