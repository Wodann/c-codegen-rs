use pretty::Pretty;

use crate::{pretty::impl_display_via_pretty, Expression, Identifier};

#[derive(Clone)]
pub struct Definition {
    pub name: Option<Identifier>,
    pub values: Vec<(Identifier, Option<Expression>)>,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Definition
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let builder = allocator.text("enum").append(allocator.space());

        let builder = if let Some(name) = self.name {
            builder
                .append(allocator.text(name.to_string()))
                .append(allocator.space())
        } else {
            builder
        };

        builder
            .append(allocator.text("{"))
            .append(allocator.intersperse(
                self.values.into_iter().map(|(identifier, value)| {
                    let builder = allocator.text(identifier.to_string());

                    if let Some(value) = value {
                        builder
                            .append(allocator.space())
                            .append(allocator.text("="))
                            .append(allocator.space())
                            .append(value.pretty(allocator))
                    } else {
                        builder
                    }
                }),
                allocator.text(",").append(allocator.space()),
            ))
            .append(allocator.text("}"))
    }
}

impl_display_via_pretty!(Definition, 80);

#[derive(Clone)]
pub enum Declaration {
    Inline {
        definition: Definition,
        variable_name: Identifier,
    },
    Typed {
        ty: Identifier,
        variable_name: Identifier,
    },
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Declaration
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let (builder, variable_name) = match self {
            Declaration::Inline {
                definition,
                variable_name,
            } => (definition.pretty(allocator), variable_name),
            Declaration::Typed { ty, variable_name } => (
                allocator
                    .text("enum")
                    .append(allocator.space())
                    .append(allocator.text(ty.to_string())),
                variable_name,
            ),
        };

        builder
            .append(allocator.space())
            .append(variable_name.to_string())
            .append(allocator.text(";"))
    }
}

impl_display_via_pretty!(Declaration, 80);

#[cfg(test)]
mod tests {
    use crate::{
        operator::{BinaryOperator, BinaryOperatorKind},
        Value,
    };

    use super::*;

    #[test]
    fn definitions() -> anyhow::Result<()> {
        let named = Definition {
            name: Some(Identifier::new("fruit")?),
            values: vec![
                (Identifier::new("grape")?, None),
                (Identifier::new("cherry")?, None),
                (Identifier::new("lemon")?, None),
                (Identifier::new("kiwi")?, None),
            ],
        }
        .to_string();
        assert_eq!(named, "enum fruit {grape, cherry, lemon, kiwi}");

        let specified_value = Definition {
            name: Some(Identifier::new("more_fruit")?),
            values: vec![
                (Identifier::new("banana")?, Some(Value::int(-17).into())),
                (Identifier::new("apple")?, None),
                (Identifier::new("blueberry")?, None),
                (Identifier::new("mango")?, None),
            ],
        }
        .to_string();
        assert_eq!(
            specified_value,
            "enum more_fruit {banana = -17, apple, blueberry, mango}"
        );

        let specified_expression = Definition {
            name: Some(Identifier::new("yet_more_fruit")?),
            values: vec![
                (Identifier::new("kumquat")?, None),
                (Identifier::new("raspberry")?, None),
                (Identifier::new("peach")?, None),
                (
                    Identifier::new("plum")?,
                    Some(
                        BinaryOperator {
                            left: Expression::Variable(Identifier::new("peach")?),
                            operator: BinaryOperatorKind::Add,
                            right: Value::int(2).into(),
                        }
                        .into(),
                    ),
                ),
            ],
        }
        .to_string();
        assert_eq!(
            specified_expression,
            "enum yet_more_fruit {kumquat, raspberry, peach, plum = peach + 2}"
        );

        Ok(())
    }

    #[test]
    fn declarations() -> anyhow::Result<()> {
        let inline = Declaration::Inline {
            definition: Definition {
                name: Some(Identifier::new("fruit")?),
                values: vec![
                    (Identifier::new("banana")?, None),
                    (Identifier::new("apple")?, None),
                    (Identifier::new("blueberry")?, None),
                    (Identifier::new("mango")?, None),
                ],
            },
            variable_name: Identifier::new("my_fruit")?,
        }
        .to_string();
        assert_eq!(
            inline,
            "enum fruit {banana, apple, blueberry, mango} my_fruit;"
        );

        let typed = Declaration::Typed {
            ty: Identifier::new("fruit")?,
            variable_name: Identifier::new("my_fruit")?,
        }
        .to_string();
        assert_eq!(typed, "enum fruit my_fruit;");

        Ok(())
    }
}
