mod array;
mod concrete;
pub mod enumeration;
mod function;
mod initializer_list;
mod integer;
pub mod member;
mod opaque;
mod pointer;
mod real;
mod scalar;
pub mod structure;
pub mod union;

use pretty::Pretty;

use crate::{macros::impl_froms, pretty::impl_display_via_pretty, statement::Typedef};

pub use self::{
    array::Array,
    concrete::ConcreteType,
    enumeration::Enum,
    function::Function,
    initializer_list::InitializerList,
    integer::{Integer, IntegerKind, StrongInt},
    opaque::OpaqueType,
    pointer::Pointer,
    real::Real,
    scalar::Scalar,
    structure::Struct,
    union::Union,
};

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
