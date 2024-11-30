use crate::{pretty::impl_display_via_pretty, Expression, Statement, Value};
use pretty::Pretty;

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
pub struct For {
    pub init: Option<Box<Statement>>,
    pub condition: Value,
    pub step: Option<Box<Statement>>,
    pub body: Vec<Statement>,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for For
where
    AnnotationT: Clone + 'a,
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let mut doc = allocator.text("for (");

        if let Some(init) = self.init {
            doc = doc.append(init.pretty(allocator));
        } else {
            doc = doc.append(allocator.text(";"));
        }

        doc = doc
            .append(allocator.text(" "))
            .append(self.condition.to_string())
            .append(allocator.text("; "));

        if let Some(step) = self.step {
            doc = doc.append(step.pretty(allocator));
        }

        doc = doc
            .append(allocator.text(") {"))
            .append(allocator.hardline());

        for stmt in self.body {
            doc = doc
                .append(stmt.pretty(allocator))
                .append(allocator.hardline());
        }

        doc.append(allocator.text("}")).nest(2)
    }
}

impl_display_via_pretty!(If, 80);
impl_display_via_pretty!(Switch, 80);
impl_display_via_pretty!(While, 80);
impl_display_via_pretty!(Do, 80);
impl_display_via_pretty!(For, 80);

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use crate::{
        function::FunctionCall,
        operator::{BinaryOperator, BinaryOperatorKind},
        Block, Expression, Identifier,
    };

    #[test]
    fn if_statement() -> anyhow::Result<()> {
        let if_stmt = If {
            condition: BinaryOperator {
                left: Expression::Variable(Identifier::new("x")?),
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
        };
        assert_eq!(
            if_stmt.to_string(),
            r#"if (x == 10)
  puts("x is 10");
else
  puts("x is not 10");
"#
        );

        let else_if_stmt = If {
            condition: BinaryOperator {
                left: Expression::Variable(Identifier::new("x")?),
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
                        left: Expression::Variable(Identifier::new("x")?),
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
        };
        // TODO: Fix extra newline
        assert_eq!(
            else_if_stmt.to_string(),
            r#"if (x == 10)
  puts("x is 10");
else if (x > 10)
  puts("x is greater than 10");
else
  puts("x is less than 10");

"#
        );

        let with_blocks = If {
            condition: BinaryOperator {
                left: Expression::Variable(Identifier::new("x")?),
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
            with_blocks.to_string(),
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

    #[test]
    fn switch_statement() -> anyhow::Result<()> {
        let switch_stmt = Switch {
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
        };

        assert_eq!(
            switch_stmt.to_string(),
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

        let empty_case = Switch {
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
        };

        assert_eq!(
            empty_case.to_string(),
            r#"switch (x)
  {
    case 0:
    case 1:
      puts("x is 0 or x is 1");
    default:
      puts("x is something else");
  }"#
        );

        let default_only = Switch {
            condition: Expression::Variable(Identifier::new("x")?),
            cases: Vec::new(),
            default: Some(vec![Expression::FunctionCall(FunctionCall {
                name: Identifier::new("puts")?,
                arguments: vec![Value::String("x is something else".to_string()).into()],
            })
            .into()]),
        };

        assert_eq!(
            default_only.to_string(),
            r#"switch (x)
  {
    default:
      puts("x is something else");
  }"#
        );

        Ok(())
    }

    #[test]
    fn while_statement() -> anyhow::Result<()> {
        let single_statement = While {
            condition: Value::int(1).into(),
            body: Expression::Variable(Identifier::new("x")?).into(),
        };
        assert_eq!(
            single_statement.to_string(),
            r#"while (1)
  x;"#
        );

        let block = While {
            condition: Value::int(1).into(),
            body: Block {
                statements: vec![Expression::Variable(Identifier::new("x")?).into()],
            }
            .into(),
        };
        assert_eq!(
            block.to_string(),
            r#"while (1)
{
  x;
}"#
        );

        let block_in_block = While {
            condition: Value::int(1).into(),
            body: Block {
                statements: vec![Block {
                    statements: vec![Expression::Variable(Identifier::new("x")?).into()],
                }
                .into()],
            }
            .into(),
        };
        assert_eq!(
            block_in_block.to_string(),
            r#"while (1)
{
  {
    x;
  }
}"#
        );

        Ok(())
    }

    #[test]
    fn do_statement() -> anyhow::Result<()> {
        let single_statement = Do {
            body: Expression::Variable(Identifier::new("x")?).into(),
            condition: Value::int(1).into(),
        };
        assert_eq!(
            single_statement.to_string(),
            r#"do
  x;
while (1);"#
        );

        let block = Do {
            body: Block {
                statements: vec![Expression::Variable(Identifier::new("x")?).into()],
            }
            .into(),
            condition: Value::int(1).into(),
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

    //     #[test]
    //     fn for_statement() -> anyhow::Result<()> {
    //         let for_stmt = For {
    //             init: Some(Box::new(Statement::Expression(Expression::Variable(
    //                 Identifier::new("i")?,
    //             )))),
    //             condition: Value::int(1),
    //             step: Some(Box::new(Statement::Expression(Expression::Variable(
    //                 Identifier::new("i")?,
    //             )))),
    //             body: vec![Statement::Expression(Expression::Variable(
    //                 Identifier::new("x")?,
    //             ))],
    //         };

    //         assert_eq!(
    //             for_stmt.to_string(),
    //             r#"for (i; 1; i) {
    //   x;
    // }"#
    //         );

    //         Ok(())
    //     }
}
