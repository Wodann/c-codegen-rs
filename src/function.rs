use pretty::Pretty;

use crate::{pretty::impl_display_via_pretty, Block, Identifier, Type};

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
        CStatement, Expression, Value,
    };

    use super::*;

    // Source: https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Function-Declarations
    #[test]
    fn declaration_with_two_parameters() -> anyhow::Result<()> {
        let generated = Declaration {
            is_static: false,
            name: Identifier::new("foo")?,
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
            name: Identifier::new("foo")?,
            return_ty: Type::int(),
            parameters: vec![
                (Type::int(), Some(Identifier::new("x")?)),
                (Type::double(), Some(Identifier::new("y")?)),
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
            name: Identifier::new("add_values")?,
            return_ty: Type::int(),
            parameters: vec![
                (Type::int(), Identifier::new("x")?),
                (Type::int(), Identifier::new("y")?),
            ],
            body: Block {
                statements: vec![CStatement::ReturnStatement(Some(
                    BinaryOperator {
                        left: Expression::Variable(Identifier::new("x")?),
                        operator: BinaryOperatorKind::Add,
                        right: Expression::Variable(Identifier::new("y")?),
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
            name: Identifier::new("foo")?,
            return_ty: Type::int(),
            parameters: vec![(Type::int(), Identifier::new("x")?)],
            body: Block {
                statements: vec![CStatement::ReturnStatement(Some(
                    BinaryOperator {
                        left: Expression::Variable(Identifier::new("x")?),
                        operator: BinaryOperatorKind::Add,
                        right: Value::int(42).into(),
                    }
                    .into(),
                ))],
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
