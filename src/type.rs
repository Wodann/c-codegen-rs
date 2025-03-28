mod array;
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

use crate::{
    macros::impl_froms, pretty::impl_display_via_pretty, statement::Typedef, FunctionDeclaration,
};

pub use self::{
    array::Array,
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
    Array(Array),
    Char,
    Enum(Enum),
    Integer(Integer),
    Function(Box<FunctionDeclaration>),
    Pointer(Pointer),
    Real(Real),
    Size,
    StrongInt(StrongInt),
    Struct(Struct),
    Typedef(Box<Typedef>),
    Union(Union),
    Void,
}

impl_froms!(Type: Array, Enum, Integer, Pointer, Real, Struct, StrongInt, box Typedef, Union);

impl From<FunctionDeclaration> for Type {
    fn from(value: FunctionDeclaration) -> Self {
        Self::Function(Box::new(value))
    }
}

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

    pub fn base_type(&self) -> Type {
        match self {
            Type::Array(array) => array.base_type(),
            ty => ty.clone(),
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
        if let Type::Array(array) = self {
            array.pretty_dimensions(allocator)
        } else {
            allocator.nil()
        }
    }
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Type
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        match self {
            Type::Array(array) => array.pretty(allocator),
            Type::Char => allocator.text("char"),
            Type::Enum(enumeration) => enumeration.pretty(allocator),
            Type::Function(function) => function.pretty(allocator),
            Type::Integer(integer) => allocator.text(integer.to_string()),
            Type::Pointer(pointer) => allocator.text(pointer.to_string()),
            Type::Real(ty) => allocator.text(ty.to_string()),
            Type::Size => allocator.text("size_t"),
            Type::StrongInt(integer) => allocator.text(integer.to_string()),
            Type::Struct(structure) => structure.pretty(allocator),
            Type::Typedef(typedef) => allocator.text(typedef.alias),
            Type::Union(union) => union.pretty(allocator),
            Type::Void => allocator.text("void"),
        }
    }
}

impl_display_via_pretty!(Type, 80);

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
