use pretty::Pretty;

use crate::{Expression, Identifier};

#[derive(Clone, Debug)]
pub enum InitializerList {
    // TODO: Use a map that maintains insertion order to avoid duplication
    Named(Vec<(Identifier, Expression)>),
    Ordered(Vec<Expression>),
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for InitializerList
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let builder = allocator.text("{").append(allocator.space());

        let builder = match self {
            InitializerList::Named(mapping) => builder.append(allocator.intersperse(
                mapping.into_iter().map(|(member, initializer)| {
                    allocator
                        .text(".")
                        .append(allocator.text(member.to_string()))
                        .append(allocator.space())
                        .append(allocator.text("="))
                        .append(allocator.space())
                        .append(initializer.pretty(allocator))
                }),
                allocator.text(",").append(allocator.space()),
            )),
            InitializerList::Ordered(initializers) => builder.append(
                allocator.intersperse(
                    initializers
                        .into_iter()
                        .map(|initializer| initializer.pretty(allocator)),
                    allocator.text(",").append(allocator.space()),
                ),
            ),
        };

        builder
            .append(allocator.space())
            .append(allocator.text("}"))
    }
}
