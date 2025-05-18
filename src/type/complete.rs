use pretty::Pretty;

use crate::{macros::impl_froms, pretty::impl_display_via_pretty, Typedef};

use super::{
    enumeration::Definition as Enum, structure::Definition as Struct, union::Definition as Union,
    Array, Integer, IntegerKind, Pointer, Real, StrongInt,
};

/// Source
///
/// https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Data-Types
#[derive(Clone, Debug)]
pub enum CompleteType {
    Array(Array),
    Char,
    Enum(Enum),
    Integer(Integer),
    Pointer(Box<Pointer>),
    Real(Real),
    Size,
    StrongInt(StrongInt),
    Struct(Struct),
    Typedef(Box<Typedef>),
    Union(Union),
    Void,
}

impl_froms!(CompleteType: Array, Enum, Integer, box Pointer, Real, Struct, StrongInt, box Typedef, Union);

impl CompleteType {
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

    pub const fn unsigned_char() -> Self {
        Self::Integer(Integer {
            kind: IntegerKind::Char,
            is_signed: false,
        })
    }

    pub const fn unsigned_int() -> Self {
        Self::Integer(Integer {
            kind: IntegerKind::Int,
            is_signed: false,
        })
    }
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for CompleteType
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        match self {
            Self::Array(array) => array.pretty(allocator),
            Self::Char => allocator.text("char"),
            Self::Enum(enumeration) => enumeration.pretty(allocator),
            Self::Integer(integer) => allocator.text(integer.to_string()),
            Self::Pointer(pointer) => allocator.text(pointer.to_string()),
            Self::Real(ty) => allocator.text(ty.to_string()),
            Self::Size => allocator.text("size_t"),
            Self::StrongInt(integer) => allocator.text(integer.to_string()),
            Self::Struct(structure) => structure.pretty(allocator),
            Self::Typedef(typedef) => allocator.text(typedef.alias),
            Self::Union(union) => union.pretty(allocator),
            Self::Void => allocator.text("void"),
        }
    }
}

impl_display_via_pretty!(CompleteType, 80);
