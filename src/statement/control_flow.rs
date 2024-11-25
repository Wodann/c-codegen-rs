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
                    let is_block = else_statement.is_block();

                    let else_statement = allocator
                        .hardline()
                        .append(else_statement.pretty(allocator));

                    if is_block {
                        else_statement
                    } else {
                        else_statement.nest(2)
                    }
                })
                .append(allocator.hardline());
        }

        builder
    }
}

#[derive(Clone)]
pub struct Switch {
    pub expression: Value,
    pub cases: Vec<(Value, Vec<Statement>)>,
    pub default: Option<Vec<Statement>>,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Switch
where
    AnnotationT: Clone + 'a,
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let mut doc = allocator
            .text("switch (")
            .append(self.expression.to_string())
            .append(allocator.text(") {"))
            .append(allocator.hardline());

        for (case, block) in self.cases {
            doc = doc
                .append(allocator.text("case "))
                .append(case.to_string())
                .append(allocator.text(":"))
                .append(allocator.hardline());

            for stmt in block {
                doc = doc
                    .append(stmt.pretty(allocator))
                    .append(allocator.hardline());
            }
            doc = doc
                .append(allocator.text("break;"))
                .append(allocator.hardline());
        }

        if let Some(block) = self.default {
            doc = doc
                .append(allocator.text("default:"))
                .append(allocator.hardline());

            for stmt in block {
                doc = doc
                    .append(stmt.pretty(allocator))
                    .append(allocator.hardline());
            }
        }

        doc.append(allocator.text("}")).nest(2)
    }
}

#[derive(Clone)]
pub struct While {
    pub condition: Value,
    pub body: Vec<Statement>,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for While
where
    AnnotationT: Clone + 'a,
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let mut doc = allocator
            .text("while (")
            .append(self.condition.to_string())
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

#[derive(Clone)]
pub struct Do {
    pub body: Vec<Statement>,
    pub condition: Value,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Do
where
    AnnotationT: Clone + 'a,
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let mut doc = allocator.text("do {").append(allocator.hardline());

        for stmt in self.body {
            doc = doc
                .append(stmt.pretty(allocator))
                .append(allocator.hardline());
        }

        doc.append(allocator.text("} while ("))
            .append(self.condition.to_string())
            .append(allocator.text(");"))
            .nest(2)
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

    //     #[test]
    //     fn switch_statement() -> anyhow::Result<()> {
    //         let switch_stmt = Switch {
    //             expression: Value::int(1),
    //             cases: vec![(
    //                 Value::int(1),
    //                 vec![Statement::Expression(Expression::Variable(
    //                     Identifier::new("x")?,
    //                 ))],
    //             )],
    //             default: Some(vec![Statement::Expression(Expression::Variable(
    //                 Identifier::new("y")?,
    //             ))]),
    //         };

    //         assert_eq!(
    //             switch_stmt.to_string(),
    //             r#"switch (1) {
    //   case 1:
    //     x;
    //     break;
    //   default:
    //     y;
    // }"#
    //         );

    //         Ok(())
    //     }

    //     #[test]
    //     fn while_statement() -> anyhow::Result<()> {
    //         let while_stmt = While {
    //             condition: Value::int(1),
    //             body: vec![Statement::Expression(Expression::Variable(
    //                 Identifier::new("x")?,
    //             ))],
    //         };

    //         assert_eq!(
    //             while_stmt.to_string(),
    //             r#"while (1) {
    //   x;
    // }"#
    //         );

    //         Ok(())
    //     }

    //     #[test]
    //     fn do_statement() -> anyhow::Result<()> {
    //         let do_stmt = Do {
    //             body: vec![Statement::Expression(Expression::Variable(
    //                 Identifier::new("x")?,
    //             ))],
    //             condition: Value::int(1),
    //         };

    //         assert_eq!(
    //             do_stmt.to_string(),
    //             r#"do {
    //   x;
    // } while (1);"#
    //         );

    //         Ok(())
    //     }

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
