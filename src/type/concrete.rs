use pretty::Pretty;

use crate::{macros::impl_froms, pretty::impl_display_via_pretty};

use super::{
    Array, Enum, Integer, IntegerKind, OpaqueType, Pointer, Real, StrongInt, Struct, Typedef, Union,
};

/// Source
///
/// https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Data-Types
#[derive(Clone)]
pub enum ConcreteType {
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

impl_froms!(ConcreteType: Array, Enum, Integer, box Pointer, Real, Struct, StrongInt, box Typedef, Union);

impl ConcreteType {
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

    /// Returns the fundamental type of the instance, after stripping away all type constructors (like pointers and arrays).
    ///
    /// # Examples
    ///
    /// - For `int[3][4]`, it returns `int`.
    /// - For `void (*)(int, int)`, it returns `void (int, int)`.
    pub fn base_type(&self) -> OpaqueType {
        match self {
            ConcreteType::Array(array) => array.base_type(),
            ConcreteType::Pointer(pointer) => pointer.base_type(),
            ty => OpaqueType::ConcreteType(ty.clone()),
        }
    }

    /// Pretty prints the dimensions of the array type, if it is an array.
    pub(crate) fn pretty_dimensions<'a, AllocatorT, AnnotationT>(
        &self,
        allocator: &'a AllocatorT,
    ) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT>
    where
        AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
        AllocatorT::Doc: Clone,
        AnnotationT: Clone + 'a,
    {
        if let ConcreteType::Array(array) = self {
            array.pretty_dimensions(allocator)
        } else {
            allocator.nil()
        }
    }
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for ConcreteType
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        match self {
            ConcreteType::Array(array) => array.pretty(allocator),
            ConcreteType::Char => allocator.text("char"),
            ConcreteType::Enum(enumeration) => enumeration.pretty(allocator),
            ConcreteType::Integer(integer) => allocator.text(integer.to_string()),
            ConcreteType::Pointer(pointer) => allocator.text(pointer.to_string()),
            ConcreteType::Real(ty) => allocator.text(ty.to_string()),
            ConcreteType::Size => allocator.text("size_t"),
            ConcreteType::StrongInt(integer) => allocator.text(integer.to_string()),
            ConcreteType::Struct(structure) => structure.pretty(allocator),
            ConcreteType::Typedef(typedef) => allocator.text(typedef.alias),
            ConcreteType::Union(union) => union.pretty(allocator),
            ConcreteType::Void => allocator.text("void"),
        }
    }
}

impl_display_via_pretty!(ConcreteType, 80);
