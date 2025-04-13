use pretty::Pretty;

use crate::{macros::impl_froms, pretty::impl_display_via_pretty, Identifier};

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

    /// Returns the innermost element type of the array, or otherwise the type itself.
    /// This is useful for determining the type of elements in a multi-dimensional array.
    ///
    /// # Examples
    ///
    /// - For `int`, it returns `int`.
    /// - For `int[3][4]`, it returns `int`.
    ///
    pub fn innermost_element_type(&self) -> ConcreteType {
        match self {
            ConcreteType::Array(array) => array.innermost_element_type(),
            ty => ty.clone(),
        }
    }

    pub fn pretty_definition<'a, 's, AllocatorT, AnnotationT>(
        self,
        alias: Identifier,
        allocator: &'a AllocatorT,
    ) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT>
    where
        AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
        AllocatorT::Doc: Clone,
        AnnotationT: Clone + 'a,
    {
        let base_type = self.base_type();
        match self {
            ConcreteType::Array(array) => {
                let alias = allocator
                    .text(alias)
                    .append(array.pretty_dimensions(allocator));

                if let OpaqueType::Function(function) = base_type {
                    let mut builder = function.pretty_signature_start(allocator);

                    if let ConcreteType::Pointer(pointer) = *array.element_type {
                        builder = builder.append(pointer.pretty_pointers(allocator));
                    }

                    builder
                        .append(alias)
                        .append(function.pretty_signature_end(allocator))
                } else {
                    array
                        .innermost_element_type()
                        .pretty(allocator)
                        .append(allocator.space())
                        .append(alias)
                }
            }
            ConcreteType::Pointer(pointer) => {
                if let OpaqueType::Function(function) = base_type {
                    function
                        .pretty_signature_start(allocator)
                        .append(pointer.pretty_pointers(allocator))
                        .append(allocator.text(alias))
                        .append(function.pretty_signature_end(allocator))
                } else {
                    pointer
                        .pretty(allocator)
                        .append(allocator.space())
                        .append(allocator.text(alias))
                }
            }
            ty => ty
                .pretty(allocator)
                .append(allocator.space())
                .append(allocator.text(alias)),
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
