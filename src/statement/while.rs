use pretty::Pretty;

use crate::Expression;

use super::{impl_display_via_pretty, Statement};

#[derive(Clone)]
pub struct While {
    pub condition: Expression,
    pub body: Statement,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for While
where
    AnnotationT: Clone + 'a,
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let while_condition = allocator
            .text("while")
            .append(allocator.space())
            .append("(")
            .append(self.condition.pretty(allocator))
            .append(allocator.text(")"))
            .append(allocator.hardline());

        let body = if matches!(self.body, Statement::Block(_)) {
            self.body.pretty(allocator)
        } else {
            self.body.pretty(allocator).indent(2)
        };

        while_condition.append(body)
    }
}

impl_display_via_pretty!(While, 80);

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use crate::{Block, Expression, Identifier, Value};

    #[test]
    fn single_statement() -> anyhow::Result<()> {
        let generated = While {
            condition: Value::int(1).into(),
            body: Expression::Variable(Identifier::new("x")?).into(),
        }
        .to_string();
        assert_eq!(
            generated,
            r#"while (1)
  x;"#
        );

        Ok(())
    }

    #[test]
    fn block() -> anyhow::Result<()> {
        let generated = While {
            condition: Value::int(1).into(),
            body: Block {
                statements: vec![Expression::Variable(Identifier::new("x")?).into()],
            }
            .into(),
        }
        .to_string();
        assert_eq!(
            generated,
            r#"while (1)
{
  x;
}"#
        );

        Ok(())
    }

    #[test]
    fn block_in_block() -> anyhow::Result<()> {
        let generated = While {
            condition: Value::int(1).into(),
            body: Block {
                statements: vec![Block {
                    statements: vec![Expression::Variable(Identifier::new("x")?).into()],
                }
                .into()],
            }
            .into(),
        }
        .to_string();
        assert_eq!(
            generated,
            r#"while (1)
{
  {
    x;
  }
}"#
        );

        Ok(())
    }
}
