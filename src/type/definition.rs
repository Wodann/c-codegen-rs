use pretty::Pretty;

use crate::{macros::impl_froms, pretty::impl_display_via_pretty};

use super::{
    enumeration::Definition as Enum, structure::Definition as Struct, union::Definition as Union,
};

#[derive(Clone, Debug)]
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
            Self::Enum(enumeration) => enumeration.pretty(allocator),
            Self::Struct(structure) => structure.pretty(allocator),
            Self::Union(union) => union.pretty(allocator),
        };

        builder.append(allocator.text(";"))
    }
}

impl_display_via_pretty!(Definition, 80);
