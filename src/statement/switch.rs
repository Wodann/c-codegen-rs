use pretty::Pretty;

use crate::Expression;

use super::{impl_display_via_pretty, Statement};

#[derive(Clone)]
pub struct Switch {
    pub condition: Expression,
    pub cases: Vec<(Expression, Vec<Statement>)>,
    pub default: Option<Vec<Statement>>,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Switch
where
    AnnotationT: Clone + 'a,
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let switch_statement = allocator
            .text("switch")
            .append(allocator.space())
            .append(allocator.text("("))
            .append(self.condition.pretty(allocator))
            .append(allocator.text(")"))
            .nest(2);

        let mut cases = allocator.nil();
        for (case, statements) in self.cases {
            let case = allocator
                .hardline()
                .append(allocator.text("case"))
                .append(allocator.space())
                .append(case.pretty(allocator))
                .append(allocator.text(":"));

            let mut body = allocator.nil();
            for statement in statements {
                body = body
                    .append(allocator.hardline())
                    .append(statement.pretty(allocator));
            }

            cases = cases.append(case.append(body.nest(2)));
        }

        if let Some(default) = self.default {
            let case = allocator.hardline().append(allocator.text("default:"));

            let mut body = allocator.nil();
            for statement in default {
                body = body
                    .append(allocator.hardline())
                    .append(statement.pretty(allocator));
            }

            cases = cases.append(case.append(body.nest(2)));
        }

        switch_statement
            .append(
                allocator
                    .hardline()
                    .append(allocator.text("{"))
                    .append(cases.nest(2))
                    .append(allocator.hardline())
                    .append(allocator.text("}")),
            )
            .nest(2)
    }
}

impl_display_via_pretty!(Switch, 80);

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use crate::{function::FunctionCall, Expression, Identifier, Value};

    #[test]
    fn switch_statement() -> anyhow::Result<()> {
        let generated = Switch {
            condition: Expression::Variable(Identifier::new("x")?),
            cases: vec![
                (
                    Value::int(0).into(),
                    vec![Expression::FunctionCall(FunctionCall {
                        name: Identifier::new("puts")?,
                        arguments: vec![Value::String("x is 0".to_string()).into()],
                    })
                    .into()],
                ),
                (
                    Value::int(1).into(),
                    vec![Expression::FunctionCall(FunctionCall {
                        name: Identifier::new("puts")?,
                        arguments: vec![Value::String("x is 1".to_string()).into()],
                    })
                    .into()],
                ),
            ],
            default: Some(vec![Expression::FunctionCall(FunctionCall {
                name: Identifier::new("puts")?,
                arguments: vec![Value::String("x is something else".to_string()).into()],
            })
            .into()]),
        }
        .to_string();

        assert_eq!(
            generated,
            r#"switch (x)
  {
    case 0:
      puts("x is 0");
    case 1:
      puts("x is 1");
    default:
      puts("x is something else");
  }"#
        );

        Ok(())
    }

    #[test]
    fn empty_case() -> anyhow::Result<()> {
        let generated = Switch {
            condition: Expression::Variable(Identifier::new("x")?),
            cases: vec![
                (Value::int(0).into(), Vec::new()),
                (
                    Value::int(1).into(),
                    vec![Expression::FunctionCall(FunctionCall {
                        name: Identifier::new("puts")?,
                        arguments: vec![Value::String("x is 0 or x is 1".to_string()).into()],
                    })
                    .into()],
                ),
            ],
            default: Some(vec![Expression::FunctionCall(FunctionCall {
                name: Identifier::new("puts")?,
                arguments: vec![Value::String("x is something else".to_string()).into()],
            })
            .into()]),
        }
        .to_string();

        assert_eq!(
            generated,
            r#"switch (x)
  {
    case 0:
    case 1:
      puts("x is 0 or x is 1");
    default:
      puts("x is something else");
  }"#
        );

        Ok(())
    }

    #[test]
    fn default_only() -> anyhow::Result<()> {
        let generated = Switch {
            condition: Expression::Variable(Identifier::new("x")?),
            cases: Vec::new(),
            default: Some(vec![Expression::FunctionCall(FunctionCall {
                name: Identifier::new("puts")?,
                arguments: vec![Value::String("x is something else".to_string()).into()],
            })
            .into()]),
        }
        .to_string();

        assert_eq!(
            generated,
            r#"switch (x)
  {
    default:
      puts("x is something else");
  }"#
        );

        Ok(())
    }
}
