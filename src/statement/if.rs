use pretty::Pretty;

use crate::Expression;

use super::{impl_display_via_pretty, Statement};

#[derive(Clone, Debug)]
pub struct If {
    pub condition: Expression,
    pub then_statement: Statement,
    pub else_statement: Option<Statement>,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for If
where
    AnnotationT: Clone + 'a,
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let mut builder = allocator
            .text("if")
            .append(allocator.space())
            .append(allocator.text("("))
            .append(self.condition.pretty(allocator))
            .append(allocator.text(")"))
            .append({
                let is_block = self.then_statement.is_block();

                let then_statement = allocator
                    .hardline()
                    .append(self.then_statement.pretty(allocator))
                    .append(allocator.hardline());

                if is_block {
                    then_statement
                } else {
                    then_statement.nest(2)
                }
            });

        if let Some(else_statement) = self.else_statement {
            builder = builder
                .append(allocator.text("else"))
                .append({
                    enum Kind {
                        Block,
                        If,
                        Other,
                    }

                    let kind = match else_statement {
                        Statement::If(_) => Kind::If,
                        Statement::Block(_) => Kind::Block,
                        _ => Kind::Other,
                    };

                    let else_statement = else_statement.pretty(allocator);
                    match kind {
                        Kind::Block => allocator.hardline().append(else_statement),
                        Kind::If => allocator.space().append(else_statement),
                        Kind::Other => allocator.hardline().append(else_statement).nest(2),
                    }
                })
                .append(allocator.hardline());
        }

        builder
    }
}

impl_display_via_pretty!(If, 80);

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use crate::{
        function::FunctionCall,
        operator::{BinaryOperator, BinaryOperatorKind},
        Block, Expression, Identifier, Value, Variable,
    };

    #[test]
    fn if_statement() -> anyhow::Result<()> {
        let generated = If {
            condition: BinaryOperator {
                left: Variable::new("x")?.into(),
                operator: BinaryOperatorKind::Eq,
                right: Value::int(10).into(),
            }
            .into(),
            then_statement: Expression::FunctionCall(FunctionCall {
                name: Identifier::new("puts")?,
                arguments: vec![Value::String("x is 10".to_string()).into()],
            })
            .into(),
            else_statement: Some(
                Expression::FunctionCall(FunctionCall {
                    name: Identifier::new("puts")?,
                    arguments: vec![Value::String("x is not 10".to_string()).into()],
                })
                .into(),
            ),
        }
        .to_string();
        assert_eq!(
            generated,
            r#"if (x == 10)
  puts("x is 10");
else
  puts("x is not 10");
"#
        );

        Ok(())
    }

    #[test]
    fn else_if_statement() -> anyhow::Result<()> {
        let generated = If {
            condition: BinaryOperator {
                left: Variable::new("x")?.into(),
                operator: BinaryOperatorKind::Eq,
                right: Value::int(10).into(),
            }
            .into(),
            then_statement: Expression::FunctionCall(FunctionCall {
                name: Identifier::new("puts")?,
                arguments: vec![Value::String("x is 10".to_string()).into()],
            })
            .into(),
            else_statement: Some(
                If {
                    condition: BinaryOperator {
                        left: Variable::new("x")?.into(),
                        operator: BinaryOperatorKind::Gt,
                        right: Value::int(10).into(),
                    }
                    .into(),
                    then_statement: Expression::FunctionCall(FunctionCall {
                        name: Identifier::new("puts")?,
                        arguments: vec![Value::String("x is greater than 10".to_string()).into()],
                    })
                    .into(),
                    else_statement: Some(
                        Expression::FunctionCall(FunctionCall {
                            name: Identifier::new("puts")?,
                            arguments: vec![Value::String("x is less than 10".to_string()).into()],
                        })
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .to_string();
        // TODO: Fix extra newline
        assert_eq!(
            generated,
            r#"if (x == 10)
  puts("x is 10");
else if (x > 10)
  puts("x is greater than 10");
else
  puts("x is less than 10");

"#
        );
        Ok(())
    }

    #[test]
    fn with_blocks() -> anyhow::Result<()> {
        let generated = If {
            condition: BinaryOperator {
                left: Variable::new("x")?.into(),
                operator: BinaryOperatorKind::Eq,
                right: Value::int(10).into(),
            }
            .into(),
            then_statement: Block {
                statements: vec![Expression::FunctionCall(FunctionCall {
                    name: Identifier::new("puts")?,
                    arguments: vec![Value::String("x is 10".to_string()).into()],
                })
                .into()],
            }
            .into(),
            else_statement: Some(
                Block {
                    statements: vec![Expression::FunctionCall(FunctionCall {
                        name: Identifier::new("puts")?,
                        arguments: vec![Value::String("x is not 10".to_string()).into()],
                    })
                    .into()],
                }
                .into(),
            ),
        };
        assert_eq!(
            generated.to_string(),
            r#"if (x == 10)
{
  puts("x is 10");
}
else
{
  puts("x is not 10");
}
"#
        );

        Ok(())
    }
}
