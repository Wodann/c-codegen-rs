use core::fmt;

pub mod enumeration;
mod initializer_list;
mod integer;
pub mod member;
mod pointer;
mod real;
mod scalar;
pub mod structure;
pub mod union;

use enumeration::Enum;
use integer::Integer;
use pretty::Pretty;
use structure::Struct;
use union::Union;

use crate::{macros::impl_froms, pretty::impl_display_via_pretty};

pub use self::{
    initializer_list::InitializerList,
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
    Enum(Enum),
    Integer(Integer),
    Pointer(Pointer),
    Real(Real),
    StrongInt(StrongInt),
    Struct(Struct),
    Union(Union),
    Void,
}

impl_froms!(Type: Enum, Integer, Pointer, Real, Struct, StrongInt, Union);

impl Type {
    pub const fn float() -> Self {
        Self::Real(Real::Float)
    }

    pub const fn double() -> Self {
        Self::Real(Real::Double)
    }

    pub const fn int() -> Self {
        Self::Integer(Integer {
            kind: IntegerKind::Int,
            is_signed: true,
        })
    }

    pub const fn unsigned_int() -> Self {
        Self::Integer(Integer {
            kind: IntegerKind::Int,
            is_signed: false,
        })
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Array(base) => write!(f, "{}[]", base),
            Type::Char => write!(f, "char"),
            Type::Enum(enumeration) => write!(f, "{enumeration}"),
            Type::Integer(integer) => write!(f, "{integer}"),
            Type::Pointer(pointer) => write!(f, "{pointer}"),
            Type::Real(ty) => write!(f, "{}", ty),
            Type::StrongInt(integer) => write!(f, "{integer}"),
            Type::Struct(structure) => write!(f, "{structure}"),
            Type::Union(union) => write!(f, "{union}"),
            Type::Void => write!(f, "void"),
        }
    }
}

#[derive(Clone)]
pub enum Definition {
    Enum(Enum),
    Struct(Struct),
    Union(Union),
}

impl_froms!(Definition: Enum, Struct, Union);

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Definition
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let builder = match self {
            Definition::Enum(enumeration) => enumeration.pretty(allocator),
            Definition::Struct(structure) => structure.pretty(allocator),
            Definition::Union(union) => union.pretty(allocator),
        };

        builder.append(allocator.text(";"))
    }
}

impl_display_via_pretty!(Definition, 80);
