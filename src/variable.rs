use pretty::Pretty;

use crate::{operator, pretty::impl_display_via_pretty, Identifier, StorageClass, Type};

#[derive(Clone)]
pub enum Definitions {
    Nil { variable_name: Identifier },
    Assignment(operator::Assignment),
}

#[derive(Clone)]
pub struct Declaration {
    pub storage_class: Option<StorageClass>,
    pub ty: Type,
    pub definitions: Vec<Definitions>,
}

// Implement Pretty for Declaration
impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Declaration
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let builder = if let Some(storage_class) = self.storage_class {
            allocator
                .text(storage_class.to_string())
                .append(allocator.space())
        } else {
            allocator.nil()
        };

        builder
            .append(allocator.text(self.ty.to_string()))
            .append(allocator.space())
            .append(
                allocator.intersperse(
                    self.definitions
                        .into_iter()
                        .map(|definition| match definition {
                            Definitions::Nil {
                                variable_name: variable,
                            } => allocator.text(variable),
                            Definitions::Assignment(assignment) => assignment.pretty(allocator),
                        }),
                    allocator.text(",").append(allocator.space()),
                ),
            )
            .append(allocator.text(";"))
    }
}

impl_display_via_pretty!(Declaration, 80);

#[cfg(test)]
mod tests {
    use crate::{Expression, Value};

    use super::*;

    #[test]
    fn multiple_initializers() -> anyhow::Result<()> {
        let generated = Declaration {
            storage_class: None,
            ty: Type::int(),
            definitions: vec![
                Definitions::Nil {
                    variable_name: Identifier::new("x")?,
                },
                Definitions::Assignment(operator::Assignment {
                    left: Expression::Variable(Identifier::new("y")?),
                    right: Expression::Value(Value::int(5)),
                }),
            ],
        }
        .to_string();

        assert_eq!(generated, "int x, y = 5;");

        Ok(())
    }
}
