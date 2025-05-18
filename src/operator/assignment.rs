use crate::{pretty::impl_display_via_pretty, Expression};
use pretty::Pretty;

/// # Source
///
/// https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Assignment-Operators
#[derive(Clone, Debug)]
pub struct Assignment {
    pub left: Expression, // This should be an l-value
    pub right: Expression,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Assignment
where
    AnnotationT: Clone + 'a,
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        self.left
            .pretty(allocator)
            .append(allocator.space())
            .append(allocator.text("="))
            .append(allocator.space())
            .append(self.right.pretty(allocator))
    }
}

impl_display_via_pretty!(Assignment, 80);
