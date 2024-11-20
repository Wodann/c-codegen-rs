use crate::{pretty::impl_display_via_pretty, Expression, Identifier};
use pretty::Pretty;
use std::fmt;

/// # Source
///
/// https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Assignment-Operators
#[derive(Clone)]
pub struct CompoundAssignment {
    pub variable: Identifier,
    pub operator: CompoundAssignmentOperator,
    pub expression: Expression,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for CompoundAssignment
where
    AnnotationT: Clone + 'a,
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        allocator
            .text(self.variable)
            .append(allocator.space())
            .append(allocator.text(self.operator.to_string()))
            .append(allocator.space())
            .append(self.expression.pretty(allocator))
    }
}

impl_display_via_pretty!(CompoundAssignment, 80);

#[derive(Clone, Copy)]
pub enum CompoundAssignmentOperator {
    Add, // +=
    Sub, // -=
    Mul, // *=
    Div, // /=
    Mod, // %=
    And, // &=
    Or,  // |=
    Xor, // ^=
    Shl, // <<=
    Shr, // >>=
}

impl fmt::Display for CompoundAssignmentOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            CompoundAssignmentOperator::Add => "+=",
            CompoundAssignmentOperator::Sub => "-=",
            CompoundAssignmentOperator::Mul => "*=",
            CompoundAssignmentOperator::Div => "/=",
            CompoundAssignmentOperator::Mod => "%=",
            CompoundAssignmentOperator::And => "&=",
            CompoundAssignmentOperator::Or => "|=",
            CompoundAssignmentOperator::Xor => "^=",
            CompoundAssignmentOperator::Shl => "<<=",
            CompoundAssignmentOperator::Shr => ">>=",
        })
    }
}
