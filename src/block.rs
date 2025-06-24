use pretty::Pretty;

use crate::{pretty::impl_display_via_pretty, Statement};

#[derive(Clone, Debug)]
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
                allocator
                    .concat(self.statements.into_iter().map(|statement| {
                        // Don't add a newline before a label, as it will be added by the label itself to guarantee the correct indentation
                        if matches!(statement, Statement::Label(_)) {
                            statement.pretty(allocator)
                        } else {
                            allocator.hardline().append(statement.pretty(allocator))
                        }
                    }))
                    .nest(2),
            )
            .append(allocator.hardline())
            .append(allocator.text("}"))
    }
}

impl_display_via_pretty!(Block, 80);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Statement, Value};

    #[test]
    fn single_line_block_indentation() -> anyhow::Result<()> {
        let block = Block {
            statements: vec![Statement::Expression(Value::signed_integer(42).into())],
        };
        assert_eq!(
            block.to_string(),
            r#"{
  42;
}"#
        );
        Ok(())
    }

    #[test]
    fn multi_line_block_indentation() -> anyhow::Result<()> {
        let block = Block {
            statements: vec![
                Statement::Expression(Value::signed_integer(1).into()),
                Statement::Expression(Value::signed_integer(2).into()),
                Statement::Expression(Value::signed_integer(3).into()),
            ],
        };
        assert_eq!(
            block.to_string(),
            r#"{
  1;
  2;
  3;
}"#
        );
        Ok(())
    }

    #[test]
    fn nested_block_indentation() -> anyhow::Result<()> {
        let inner_block = Block {
            statements: vec![Statement::Expression(Value::signed_integer(42).into())],
        };
        let outer_block = Block {
            statements: vec![
                Statement::Expression(Value::signed_integer(1).into()),
                Statement::Block(inner_block),
                Statement::Expression(Value::signed_integer(2).into()),
            ],
        };
        assert_eq!(
            outer_block.to_string(),
            r#"{
  1;
  {
    42;
  }
  2;
}"#
        );
        Ok(())
    }
}
