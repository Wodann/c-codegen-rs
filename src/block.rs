use pretty::Pretty;

use crate::{pretty::impl_display_via_pretty, Statement};

#[derive(Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Block
where
    AnnotationT: Clone + 'a,
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        allocator
            .text("{")
            .append(
                allocator.hardline().append(
                    allocator.intersperse(
                        self.statements
                            .into_iter()
                            .map(|statement| allocator.text(statement.to_string())),
                        allocator.hardline(),
                    ),
                ),
            )
            .nest(2)
            .append(allocator.hardline())
            .append(allocator.text("}"))
    }
}

impl_display_via_pretty!(Block, 80);
