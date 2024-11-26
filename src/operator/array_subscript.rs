use pretty::Pretty;

use crate::Expression;

#[derive(Clone)]
pub struct ArraySubscript {
    pub array: Expression,
    pub index: Expression,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for ArraySubscript
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        self.array
            .pretty(allocator)
            .append(allocator.text("["))
            .append(self.index.pretty(allocator))
            .append(allocator.text("]"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        function::FunctionCall,
        operator::{BinaryOperator, BinaryOperatorKind},
        Identifier, Statement, Value,
    };

    #[test]
    fn basic() -> anyhow::Result<()> {
        let array_access =
            Statement::Expression(Expression::ArraySubscript(Box::new(ArraySubscript {
                array: Expression::Variable(Identifier::new("my_array")?),
                index: Value::int(0).into(),
            })));

        assert_eq!(array_access.to_string(), "my_array[0];");
        Ok(())
    }

    #[test]
    fn complex() -> anyhow::Result<()> {
        // Test with a function call as the array expression: get_array()[0]
        let function_array_access =
            Statement::Expression(Expression::ArraySubscript(Box::new(ArraySubscript {
                array: FunctionCall {
                    name: Identifier::new("get_array")?,
                    arguments: vec![],
                }
                .into(),
                index: Value::int(0).into(),
            })));
        assert_eq!(function_array_access.to_string(), "get_array()[0];");

        // Test with a complex index expression: arr[i + j * 2]
        let complex_index_access =
            Statement::Expression(Expression::ArraySubscript(Box::new(ArraySubscript {
                array: Expression::Variable(Identifier::new("arr")?),
                index: Expression::BinaryOperator(Box::new(BinaryOperator {
                    left: Expression::Variable(Identifier::new("i")?),
                    operator: BinaryOperatorKind::Add,
                    right: Expression::BinaryOperator(Box::new(BinaryOperator {
                        left: Expression::Variable(Identifier::new("j")?),
                        operator: BinaryOperatorKind::Mul,
                        right: Value::int(2).into(),
                    })),
                })),
            })));
        assert_eq!(complex_index_access.to_string(), "arr[i + j * 2];");

        Ok(())
    }
}
