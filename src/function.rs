use pretty::Pretty;

use crate::{
    pretty::impl_display_via_pretty, r#type::Function, Block, ConcreteType, Expression, Identifier,
};

#[derive(Clone, Debug)]
pub struct FunctionCall {
    pub callee: Expression,
    pub arguments: Vec<Expression>,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for FunctionCall
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        self.callee
            .pretty(allocator)
            .append(allocator.text("("))
            .append(allocator.intersperse(
                self.arguments.into_iter().map(|arg| arg.pretty(allocator)),
                allocator.text(",").append(allocator.space()),
            ))
            .append(allocator.text(")"))
    }
}

#[derive(Clone, Debug)]
pub struct FunctionParameter {
    pub ty: ConcreteType,
    pub name: Option<Identifier>,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for FunctionParameter
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let builder = allocator.text(self.ty.to_string());
        if let Some(name) = self.name {
            builder
                .append(allocator.space())
                .append(allocator.text(name))
        } else {
            builder
        }
    }
}

/// # Source
///
/// https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Function-Declarations
#[derive(Clone, Debug)]
pub struct Declaration {
    pub is_static: bool,
    pub name: Identifier,
    pub ty: Function,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Declaration
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let builder = if self.is_static {
            allocator.text("static").append(allocator.space())
        } else {
            allocator.nil()
        };

        let return_type = self.ty.pretty_return_type(allocator);
        let parameters = self.ty.pretty_parameters(allocator);

        builder
            .append(return_type)
            .append(allocator.text(self.name))
            .append(allocator.space())
            .append(parameters)
            .append(allocator.text(";"))
    }
}

impl_display_via_pretty!(Declaration, 80);

/// # Source
///
/// https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Function-Definitions
#[derive(Clone, Debug)]
pub struct Definition {
    pub is_static: bool,
    pub name: Identifier,
    pub parameters: Vec<(ConcreteType, Identifier)>,
    pub return_ty: ConcreteType,
    pub body: Block,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Definition
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let builder = if self.is_static {
            allocator.text("static").append(allocator.space())
        } else {
            allocator.nil()
        };

        builder
            .append(allocator.text(self.return_ty.to_string()))
            .append(allocator.hardline())
            .append(allocator.text(self.name))
            .append(allocator.space())
            .append(allocator.text("("))
            .append(allocator.intersperse(
                self.parameters.into_iter().map(|(ty, name)| {
                    allocator
                        .text(ty.to_string())
                        .append(allocator.space())
                        .append(allocator.text(name))
                }),
                allocator.text(",").append(allocator.space()),
            ))
            .append(allocator.text(")"))
            .append(allocator.space())
            .append(self.body.pretty(allocator))
    }
}

impl_display_via_pretty!(Definition, 80);

#[cfg(test)]
mod tests {
    use crate::{
        operator::{ArraySubscript, BinaryOperator, BinaryOperatorKind},
        statement::Return,
        Value, Variable,
    };

    use super::*;

    // Source: https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Function-Declarations
    #[test]
    fn declaration_with_two_parameters() -> anyhow::Result<()> {
        let generated = Declaration {
            is_static: false,
            name: Identifier::new("foo")?,
            ty: Function {
                return_ty: ConcreteType::int(),
                parameters: vec![
                    FunctionParameter {
                        ty: ConcreteType::int(),
                        name: None,
                    },
                    FunctionParameter {
                        ty: ConcreteType::double(),
                        name: None,
                    },
                ],
            },
        }
        .to_string();

        assert_eq!(generated, "int foo (int, double);");

        Ok(())
    }

    // Source: https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Function-Declarations
    #[test]
    fn declaration_with_parameter_name() -> anyhow::Result<()> {
        let generated = Declaration {
            is_static: false,
            name: Identifier::new("foo")?,
            ty: Function {
                return_ty: ConcreteType::int(),
                parameters: vec![
                    FunctionParameter {
                        ty: ConcreteType::int(),
                        name: Some(Identifier::new("x")?),
                    },
                    FunctionParameter {
                        ty: ConcreteType::double(),
                        name: Some(Identifier::new("y")?),
                    },
                ],
            },
        }
        .to_string();

        assert_eq!(generated, "int foo (int x, double y);");

        Ok(())
    }

    // Source: https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Function-Definitions
    #[test]
    fn definition_with_integer_sum() -> anyhow::Result<()> {
        let generated = Definition {
            is_static: false,
            name: Identifier::new("add_values")?,
            return_ty: ConcreteType::int(),
            parameters: vec![
                (ConcreteType::int(), Identifier::new("x")?),
                (ConcreteType::int(), Identifier::new("y")?),
            ],
            body: Block {
                statements: vec![Return {
                    expression: Some(
                        BinaryOperator {
                            left: Variable::new("x")?.into(),
                            operator: BinaryOperatorKind::Add,
                            right: Variable::new("y")?.into(),
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
add_values (int x, int y) {
  return x + y;
}"#
        );

        Ok(())
    }

    // Source: https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Static-Functions
    #[test]
    fn static_function() -> anyhow::Result<()> {
        let generated = Definition {
            is_static: true,
            name: Identifier::new("foo")?,
            return_ty: ConcreteType::int(),
            parameters: vec![(ConcreteType::int(), Identifier::new("x")?)],
            body: Block {
                statements: vec![Return {
                    expression: Some(
                        BinaryOperator {
                            left: Variable::new("x")?.into(),
                            operator: BinaryOperatorKind::Add,
                            right: Value::signed_integer(42).into(),
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
            r#"static int
foo (int x) {
  return x + 42;
}"#
        );

        Ok(())
    }

    #[test]
    fn function_call() -> anyhow::Result<()> {
        let function_name = Expression::FunctionCall(Box::new(FunctionCall {
            callee: Variable::new("calculate_sum")?.into(),
            arguments: vec![
                Value::signed_integer(5).into(),
                Value::signed_integer(10).into(),
            ],
        }))
        .to_string();

        assert_eq!(function_name, "calculate_sum(5, 10)");

        let expression = Expression::FunctionCall(Box::new(FunctionCall {
            callee: ArraySubscript {
                array: Variable::new("my_array")?.into(),
                index: Value::signed_integer(0).into(),
            }
            .into(),
            arguments: vec![
                Value::signed_integer(5).into(),
                Value::signed_integer(10).into(),
            ],
        }))
        .to_string();

        assert_eq!(expression, "my_array[0](5, 10)");

        Ok(())
    }
}
