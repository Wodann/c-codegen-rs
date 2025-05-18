use pretty::Pretty;

use crate::{Expression, VariableDeclaration};

use super::{impl_display_via_pretty, impl_froms, Statement};

#[derive(Clone, Debug)]
pub enum ForDeclaration {
    Expression(Expression),
    VariableDeclaration(VariableDeclaration),
}

impl_froms!(ForDeclaration: VariableDeclaration, Expression);

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for ForDeclaration
where
    AnnotationT: Clone + 'a,
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        match self {
            ForDeclaration::Expression(expression) => expression.pretty(allocator),
            ForDeclaration::VariableDeclaration(declaration) => declaration.pretty(allocator),
        }
    }
}

#[derive(Clone, Debug)]
pub struct For {
    pub init: Option<ForDeclaration>,
    pub condition: Expression,
    pub step: Option<Expression>,
    pub body: Statement,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for For
where
    AnnotationT: Clone + 'a,
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let init = if let Some(init) = self.init {
            init.pretty(allocator)
        } else {
            allocator.nil()
        };

        let step = if let Some(step) = self.step {
            step.pretty(allocator)
        } else {
            allocator.nil()
        };

        let for_condition = init
            .append(allocator.text(";"))
            .append(allocator.space())
            .append(self.condition.pretty(allocator))
            .append(allocator.text(";"))
            .append(allocator.space())
            .append(step)
            .parens();

        let body = if matches!(self.body, Statement::Block(_)) {
            self.body.pretty(allocator)
        } else {
            self.body.pretty(allocator).indent(2)
        };

        allocator
            .text("for")
            .append(allocator.space())
            .append(for_condition)
            .append(allocator.hardline())
            .append(body)
    }
}

impl_display_via_pretty!(For, 80);

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use crate::{
        function::FunctionCall,
        operator::{
            Assignment, BinaryOperator, BinaryOperatorKind, CommaOperator, CompoundAssignment,
            CompoundAssignmentOperator, PrefixOperator, PrefixOperatorKind,
        },
        Block, ConcreteType, Expression, Identifier, Value, Variable,
    };

    #[test]
    fn single_statement() -> anyhow::Result<()> {
        let generated = For {
            init: Some(Expression::Variable(Variable::new("i")?).into()),
            condition: Value::int(1).into(),
            step: Some(Variable::new("i")?.into()),
            body: Expression::Variable(Variable::new("x")?).into(),
        }
        .to_string();
        assert_eq!(
            generated,
            r#"for (i; 1; i)
  x;"#
        );

        Ok(())
    }

    #[test]
    fn block() -> anyhow::Result<()> {
        let generated = For {
            init: Some(Expression::Variable(Variable::new("i")?).into()),
            condition: Value::int(1).into(),
            step: Some(Variable::new("i")?.into()),
            body: Block {
                statements: vec![Expression::Variable(Variable::new("x")?).into()],
            }
            .into(),
        }
        .to_string();
        assert_eq!(
            generated,
            r#"for (i; 1; i)
{
  x;
}"#
        );

        Ok(())
    }

    #[test]
    fn init_condition_step() -> anyhow::Result<()> {
        let generated = For {
            init: Some(ForDeclaration::VariableDeclaration(VariableDeclaration {
                storage_class: None,
                ty: ConcreteType::Size,
                identifier: Identifier::new("i")?,
                initializer: Some(Value::int(0).into()),
            })),
            condition: BinaryOperator {
                left: Variable::new("i")?.into(),
                operator: BinaryOperatorKind::Lt,
                right: Value::int(5).into(),
            }
            .into(),
            step: Some(
                PrefixOperator {
                    operand: Variable::new("i")?.into(),
                    operator: PrefixOperatorKind::Increment,
                }
                .into(),
            ),
            body: Expression::Variable(Variable::new("x")?).into(),
        }
        .to_string();
        assert_eq!(
            generated,
            r#"for (size_t i = 0; i < 5; ++i)
  x;"#
        );

        Ok(())
    }

    #[test]
    fn complex_for_loop() -> anyhow::Result<()> {
        // Create initialization expressions x = 1, y = 10
        let x_init = Assignment {
            left: Variable::new("x")?.into(),
            right: Value::int(1).into(),
        };
        let y_init = Assignment {
            left: Variable::new("y")?.into(),
            right: Value::int(10).into(),
        };
        let init = CommaOperator {
            left: x_init.into(),
            right: y_init.into(),
        };

        // Create condition x <= 10 && y >= 1
        let x_cond = BinaryOperator {
            left: Variable::new("x")?.into(),
            operator: BinaryOperatorKind::Le,
            right: Value::int(10).into(),
        };
        let y_cond = BinaryOperator {
            left: Variable::new("y")?.into(),
            operator: BinaryOperatorKind::Ge,
            right: Value::int(1).into(),
        };
        let condition = BinaryOperator {
            left: x_cond.into(),
            operator: BinaryOperatorKind::And,
            right: y_cond.into(),
        };

        // Create increment expressions x+=2, y--
        let x_inc = CompoundAssignment {
            left: Variable::new("x")?.into(),
            operator: CompoundAssignmentOperator::Add,
            right: Value::int(2).into(),
        };
        let y_dec = PrefixOperator {
            operand: Variable::new("y")?.into(),
            operator: PrefixOperatorKind::Decrement,
        };
        let step = CommaOperator {
            left: x_inc.into(),
            right: y_dec.into(),
        }
        .into();

        // Create printf statement
        let printf_call = FunctionCall {
            name: Identifier::new("printf")?,
            arguments: vec![
                Value::String("%d %d\\n".to_string()).into(),
                Variable::new("x")?.into(),
                Variable::new("y")?.into(),
            ],
        };

        let generated = For {
            init: Some(Expression::from(init).into()),
            condition: condition.into(),
            step: Some(step),
            body: Statement::Expression(printf_call.into()),
        }
        .to_string();

        assert_eq!(
            generated,
            r#"for (x = 1, y = 10; x <= 10 && y >= 1; x += 2, --y)
  printf("%d %d\n", x, y);"#
        );

        Ok(())
    }
}
