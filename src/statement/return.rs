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
        Block, Identifier, ConcreteType, Variable,
    };

    #[test]
    fn void() -> anyhow::Result<()> {
        let generated = function::Definition {
            is_static: false,
            name: Identifier::new("print_plus_five")?,
            parameters: vec![(ConcreteType::int(), Identifier::new("x")?)],
            return_ty: ConcreteType::Void,
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
            parameters: vec![(ConcreteType::int(), Identifier::new("x")?)],
            return_ty: ConcreteType::int(),
            body: Block {
                statements: vec![Return {
                    expression: Some(
                        BinaryOperator {
                            left: Variable::new("x")?.into(),
                            operator: BinaryOperatorKind::Mul,
                            right: Variable::new("x")?.into(),
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
