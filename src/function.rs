use pretty::Pretty;

use crate::{pretty::impl_display_via_pretty, Block, Type, Identifier};

/// # Source
///
/// https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Function-Declarations
#[derive(Clone)]
pub struct Declaration {
    pub is_static: bool,
    pub name: Identifier,
    pub parameters: Vec<(Type, Option<Identifier>)>,
    pub return_ty: Type,
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

        builder
            .append(allocator.text(self.return_ty.to_string()))
            .append(allocator.space())
            .append(allocator.text(self.name))
            .append(allocator.space())
            .append(allocator.text("("))
            .append(allocator.intersperse(
                self.parameters.into_iter().map(|(ty, name)| {
                    let builder = allocator.text(ty.to_string());
                    if let Some(name) = name {
                        builder
                            .append(allocator.space())
                            .append(allocator.text(name))
                    } else {
                        builder
                    }
                }),
                allocator.text(",").append(allocator.space()),
            ))
            .append(allocator.text(");"))
    }
}

impl_display_via_pretty!(Declaration, 80);

/// # Source
///
/// https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Function-Definitions
#[derive(Clone)]
pub struct Definition {
    pub is_static: bool,
    pub name: Identifier,
    pub parameters: Vec<(Type, Identifier)>,
    pub return_ty: Type,
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
            .append(allocator.hardline())
            .append(self.body.pretty(allocator))
    }
}

impl_display_via_pretty!(Definition, 80);

#[cfg(test)]
mod tests {
    use crate::{
        operator::{BinaryOperator, BinaryOperatorKind},
        CStatement, Expression,
    };

    use super::*;

    // Source: https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Function-Declarations
    #[test]
    fn declaration_with_two_parameters() -> anyhow::Result<()> {
        let generated = Declaration {
            is_static: false,
            name: "foo".to_string(),
            return_ty: Type::int(),
            parameters: vec![(Type::int(), None), (Type::double(), None)],
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
            name: "foo".to_string(),
            return_ty: Type::int(),
            parameters: vec![
                (Type::int(), Some("x".to_string())),
                (Type::double(), Some("y".to_string())),
            ],
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
            name: "add_values".to_string(),
            return_ty: Type::int(),
            parameters: vec![
                (Type::int(), "x".to_string()),
                (Type::int(), "y".to_string()),
            ],
            body: Block {
                statements: vec![CStatement::ReturnStatement(Some(
                    BinaryOperator {
                        left: Expression::Variable("x".to_string()),
                        operator: BinaryOperatorKind::Add,
                        right: Expression::Variable("y".to_string()),
                    }
                    .into(),
                ))],
            },
        }
        .to_string();

        assert_eq!(
            generated,
            r#"int
add_values (int x, int y)
{
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
            name: "foo".to_string(),
            return_ty: Type::int(),
            parameters: vec![(Type::int(), "x".to_string())],
            body: Block {
                statements: vec![CStatement::ReturnStatement(Some(Expression::Custom(
                    "x + 42".to_string(),
                )))],
            },
        }
        .to_string();

        assert_eq!(
            generated,
            r#"static int
foo (int x)
{
  return x + 42;
}"#
        );

        Ok(())
    }
}
