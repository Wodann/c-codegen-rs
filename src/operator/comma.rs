use crate::Expression;
use pretty::Pretty;

#[derive(Clone)]
pub struct CommaOperator {
    pub left: Expression,
    pub right: Expression,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for CommaOperator
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        self.left
            .pretty(allocator)
            .append(allocator.text(","))
            .append(allocator.space())
            .append(self.right.pretty(allocator))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        operator::{
            Assignment, BinaryOperator, BinaryOperatorKind, PostfixOperator, PostfixOperatorKind,
        },
        Variable,
    };

    #[test]
    fn expression() -> anyhow::Result<()> {
        // Test case from book: x++, y = x * x
        let generated = Expression::from(CommaOperator {
            left: PostfixOperator {
                operand: Variable::new("x")?.into(),
                operator: PostfixOperatorKind::Increment,
            }
            .into(),
            right: Assignment {
                left: Variable::new("y")?.into(),
                right: BinaryOperator {
                    left: Variable::new("x")?.into(),
                    operator: BinaryOperatorKind::Mul,
                    right: Variable::new("x")?.into(),
                }
                .into(),
            }
            .into(),
        })
        .to_string();

        assert_eq!(generated, "x++, y = x * x");

        Ok(())
    }
}
