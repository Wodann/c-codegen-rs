use crate::Expression;
use pretty::Pretty;

#[derive(Clone)]
pub struct Return {
    pub expression: Option<Expression>,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Return
where
    AnnotationT: Clone + 'a,
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let expression = if let Some(expression) = self.expression {
            allocator.space().append(expression.pretty(allocator))
        } else {
            allocator.nil()
        };

        allocator
            .text("return")
            .append(expression)
            .append(allocator.text(";"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        function,
        operator::{BinaryOperator, BinaryOperatorKind},
        Block, Identifier, Type,
    };

    #[test]
    fn void() -> anyhow::Result<()> {
        let generated = function::Definition {
            is_static: false,
            name: Identifier::new("print_plus_five")?,
            parameters: vec![(Type::int(), Identifier::new("x")?)],
            return_ty: Type::Void,
            body: Block {
                statements: vec![Return { expression: None }.into()],
            },
        }
        .to_string();
        assert_eq!(
            generated,
            r#"void
print_plus_five (int x) {
  return;
}"#
        );

        Ok(())
    }

    #[test]
    fn with_expression() -> anyhow::Result<()> {
        let generated = function::Definition {
            is_static: false,
            name: Identifier::new("square_value")?,
            parameters: vec![(Type::int(), Identifier::new("x")?)],
            return_ty: Type::int(),
            body: Block {
                statements: vec![Return {
                    expression: Some(
                        BinaryOperator {
                            left: Expression::Variable(Identifier::new("x")?),
                            operator: BinaryOperatorKind::Mul,
                            right: Expression::Variable(Identifier::new("x")?),
                        }
                        .into(),
                    ),
                }
                .into()],
            },
        }
        .to_string();
        assert_eq!(
            generated,
            r#"int
square_value (int x) {
  return x * x;
}"#
        );

        Ok(())
    }
}
