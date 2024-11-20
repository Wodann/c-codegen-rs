use crate::{pretty::impl_display_via_pretty, Expression, Identifier};
use pretty::Pretty;

/// # Source
///
/// https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Assignment-Operators
#[derive(Clone)]
pub struct Assignment {
    pub variable_name: Identifier,
    pub expression: Expression,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Assignment
where
    AnnotationT: Clone + 'a,
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        allocator
            .text(self.variable_name)
            .append(allocator.space())
            .append(allocator.text("="))
            .append(allocator.space())
            .append(self.expression.pretty(allocator))
    }
}

impl_display_via_pretty!(Assignment, 80);
