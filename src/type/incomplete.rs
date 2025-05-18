use pretty::Pretty;

use crate::{macros::impl_froms, pretty::impl_display_via_pretty, Identifier, Typedef};

use super::{
    enumeration::Declaration as Enum, structure::Declaration as Struct,
    union::Declaration as Union, Array, Integer, IntegerKind, OpaqueType, Pointer, Real, StrongInt,
};

/// Source
///
/// https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Data-Types
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IncompleteType {
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

impl_froms!(IncompleteType: Array, Enum, Integer, box Pointer, Real, Struct, StrongInt, box Typedef, Union);

impl IncompleteType {
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
            Self::Array(array) => array.base_type(),
            Self::Pointer(pointer) => pointer.base_type(),
            ty => OpaqueType::IncompleteType(ty.clone()),
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
    pub fn innermost_element_type(&self) -> IncompleteType {
        match self {
            IncompleteType::Array(array) => array.innermost_element_type(),
            ty => ty.clone(),
        }
    }

    /// Whether the pointer needs a trailing whitespace.
    ///
    /// This is the case for:
    /// - non-pointer types (e.g. `int x;`)
    /// - pointers for which the last pointer is constant (e.g. `int *const x;`)
    pub(crate) fn needs_trailing_whitespace(&self) -> bool {
        if let IncompleteType::Pointer(pointer) = self {
            pointer.needs_trailing_whitespace()
        } else {
            true
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
            IncompleteType::Array(array) => {
                let alias = allocator
                    .text(alias)
                    .append(array.pretty_dimensions(allocator));

                if let OpaqueType::Function(function) = base_type {
                    let mut builder = function.pretty_signature_start(allocator);

                    if let IncompleteType::Pointer(pointer) = *array.element_type {
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
            IncompleteType::Pointer(pointer) => {
                let needs_trailing_whitespace = pointer.needs_trailing_whitespace();

                if let OpaqueType::Function(function) = base_type {
                    let builder = function
                        .pretty_signature_start(allocator)
                        .append(pointer.pretty_pointers(allocator));

                    let builder = if needs_trailing_whitespace {
                        builder.append(allocator.space())
                    } else {
                        builder
                    };

                    builder
                        .append(allocator.text(alias))
                        .append(function.pretty_signature_end(allocator))
                } else {
                    let builder = pointer.pretty(allocator);

                    let builder = if needs_trailing_whitespace {
                        builder.append(allocator.space())
                    } else {
                        builder
                    };

                    builder.append(allocator.text(alias))
                }
            }
            ty => ty
                .pretty(allocator)
                .append(allocator.space())
                .append(allocator.text(alias)),
        }
    }
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for IncompleteType
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        match self {
            IncompleteType::Array(array) => array.pretty(allocator),
            IncompleteType::Char => allocator.text("char"),
            IncompleteType::Enum(enumeration) => enumeration.pretty(allocator),
            IncompleteType::Integer(integer) => allocator.text(integer.to_string()),
            IncompleteType::Pointer(pointer) => allocator.text(pointer.to_string()),
            IncompleteType::Real(ty) => allocator.text(ty.to_string()),
            IncompleteType::Size => allocator.text("size_t"),
            IncompleteType::StrongInt(integer) => allocator.text(integer.to_string()),
            IncompleteType::Struct(structure) => structure.pretty(allocator),
            IncompleteType::Typedef(typedef) => allocator.text(typedef.alias),
            IncompleteType::Union(union) => union.pretty(allocator),
            IncompleteType::Void => allocator.text("void"),
        }
    }
}

impl_display_via_pretty!(IncompleteType, 80);
