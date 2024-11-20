use crate::{pretty::impl_display_via_pretty, Identifier};
use pretty::Pretty;
use std::fmt;

/// Represents postfix increment (x++) and decrement (x--) operations
#[derive(Clone)]
pub struct PostfixOperator {
    pub variable: Identifier,
    pub operator: PostfixOperatorKind,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for PostfixOperator
where
    AnnotationT: Clone + 'a,
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        allocator
            .text(self.variable)
            .append(self.operator.to_string())
    }
}

impl_display_via_pretty!(PostfixOperator, 80);

#[derive(Clone, Copy)]
pub enum PostfixOperatorKind {
    Increment, // x++
    Decrement, // x--
}

impl fmt::Display for PostfixOperatorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            PostfixOperatorKind::Increment => "++",
            PostfixOperatorKind::Decrement => "--",
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::CStatement;

    use super::*;

    #[test]
    fn increment_decrement() -> anyhow::Result<()> {
        let increment = CStatement::Expression(
            PostfixOperator {
                variable: "x".to_string(),
                operator: PostfixOperatorKind::Increment,
            }
            .into(),
        )
        .to_string();

        assert_eq!(increment, "x++;");

        let decrement = CStatement::Expression(
            PostfixOperator {
                variable: "y".to_string(),
                operator: PostfixOperatorKind::Decrement,
            }
            .into(),
        )
        .to_string();

        assert_eq!(decrement, "y--;");

        Ok(())
    }
}
