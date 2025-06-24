use crate::{pretty::impl_display_via_pretty, Expression, Statement};
use pretty::Pretty;

#[derive(Clone, Debug)]
pub struct Do {
    pub body: Statement,
    pub condition: Expression,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Do
where
    AnnotationT: Clone + 'a,
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let body = if matches!(self.body, Statement::Block(_)) {
            self.body.pretty(allocator)
        } else {
            self.body.pretty(allocator).indent(2)
        };

        allocator
            .text("do")
            .append(allocator.hardline())
            .append(body)
            .append(allocator.hardline())
            .append(allocator.text("while"))
            .append(allocator.space())
            .append(self.condition.pretty(allocator).parens())
            .append(allocator.text(";"))
    }
}

impl_display_via_pretty!(Do, 80);

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use crate::{Block, Expression, Value, Variable};

    #[test]
    fn do_statement() -> anyhow::Result<()> {
        let single_statement = Do {
            body: Expression::Variable(Variable::new("x")?).into(),
            condition: Value::signed_integer(1).into(),
        };
        assert_eq!(
            single_statement.to_string(),
            r#"do
  x;
while (1);"#
        );

        let block = Do {
            body: Block {
                statements: vec![Expression::Variable(Variable::new("x")?).into()],
            }
            .into(),
            condition: Value::signed_integer(1).into(),
        };
        assert_eq!(
            block.to_string(),
            r#"do
{
  x;
}
while (1);"#
        );

        Ok(())
    }
}
