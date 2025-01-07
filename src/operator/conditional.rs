use crate::{pretty::impl_display_via_pretty, Expression};
use pretty::Pretty;

#[derive(Clone)]
pub struct Conditional {
    pub condition: Expression,
    pub then_branch: Expression,
    pub else_branch: Expression,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Conditional
where
    AnnotationT: Clone + 'a,
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        self.condition
            .pretty(allocator)
            .append(allocator.space())
            .append(allocator.text("?"))
            .append(allocator.space())
            .append(self.then_branch.pretty(allocator))
            .append(allocator.space())
            .append(allocator.text(":"))
            .append(allocator.space())
            .append(self.else_branch.pretty(allocator))
    }
}

impl_display_via_pretty!(Conditional, 80);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        operator::{BinaryOperator, BinaryOperatorKind},
        Value, Variable,
    };

    #[test]
    fn single() -> anyhow::Result<()> {
        let conditional = Conditional {
            condition: Expression::Parentheses(Box::new(
                BinaryOperator {
                    left: Variable::new("x")?.into(),
                    operator: BinaryOperatorKind::Eq,
                    right: Value::int(5).into(),
                }
                .into(),
            )),
            then_branch: Variable::new("y")?.into(),
            else_branch: Variable::new("z")?.into(),
        };

        assert_eq!(conditional.to_string(), "(x == 5) ? y : z");

        Ok(())
    }

    #[test]
    fn nested() -> anyhow::Result<()> {
        let conditional = Conditional {
            condition: Expression::Parentheses(Box::new(
                BinaryOperator {
                    left: Variable::new("x")?.into(),
                    operator: BinaryOperatorKind::Eq,
                    right: Value::int(5).into(),
                }
                .into(),
            )),
            then_branch: Expression::Parentheses(Box::new(
                Conditional {
                    condition: BinaryOperator {
                        left: Variable::new("y")?.into(),
                        operator: BinaryOperatorKind::Gt,
                        right: Value::int(10).into(),
                    }
                    .into(),

                    then_branch: Value::int(1).into(),
                    else_branch: Value::int(2).into(),
                }
                .into(),
            )),
            else_branch: Value::int(3).into(),
        };

        assert_eq!(conditional.to_string(), "(x == 5) ? (y > 10 ? 1 : 2) : 3");

        Ok(())
    }
}
