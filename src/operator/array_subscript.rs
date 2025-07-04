use pretty::Pretty;

use crate::Expression;

#[derive(Clone, Debug)]
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
        Identifier, Statement, Value, Variable,
    };

    #[test]
    fn basic() -> anyhow::Result<()> {
        let array_access =
            Statement::Expression(Expression::ArraySubscript(Box::new(ArraySubscript {
                array: Variable::new("my_array")?.into(),
                index: Value::signed_integer(0).into(),
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
                    callee: Identifier::new("get_array")?.into(),
                    arguments: vec![],
                }
                .into(),
                index: Value::signed_integer(0).into(),
            })));
        assert_eq!(function_array_access.to_string(), "get_array()[0];");

        // Test with a complex index expression: arr[i + j * 2]
        let complex_index_access =
            Statement::Expression(Expression::ArraySubscript(Box::new(ArraySubscript {
                array: Variable::new("arr")?.into(),
                index: Expression::BinaryOperator(Box::new(BinaryOperator {
                    left: Variable::new("i")?.into(),
                    operator: BinaryOperatorKind::Add,
                    right: Expression::BinaryOperator(Box::new(BinaryOperator {
                        left: Variable::new("j")?.into(),
                        operator: BinaryOperatorKind::Mul,
                        right: Value::signed_integer(2).into(),
                    })),
                })),
            })));
        assert_eq!(complex_index_access.to_string(), "arr[i + j * 2];");

        Ok(())
    }
}
