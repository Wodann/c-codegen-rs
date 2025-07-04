use pretty::Pretty;

use crate::{non_empty_vec::NonEmptyVec, pretty::impl_display_via_pretty, Expression, Identifier};

#[derive(Clone, Debug)]
pub enum Enum {
    Definition {
        name: Option<Identifier>,
        values: NonEmptyVec<(Identifier, Option<Expression>)>,
    },
    /// An incomplete enumeration type, only useable as pointer type. Requires a complete definition elsewhere.
    Tag { name: Identifier },
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Enum
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let builder = allocator.text("enum").append(allocator.space());

        match self {
            Enum::Definition { name, values } => {
                let builder = if let Some(name) = name {
                    builder
                        .append(allocator.text(name.to_string()))
                        .append(allocator.space())
                } else {
                    builder
                };

                builder
                    .append(allocator.text("{"))
                    .append(allocator.intersperse(
                        values.into_iter().map(|(identifier, value)| {
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
            Enum::Tag { name } => builder.append(allocator.text(name.to_string())),
        }
    }
}

impl_display_via_pretty!(Enum, 80);

#[cfg(test)]
mod tests {
    use crate::{
        operator::{BinaryOperator, BinaryOperatorKind},
        r#type::Definition,
        variable, Statement, Value, Variable,
    };

    use super::*;

    #[test]
    fn complete_definitions() -> anyhow::Result<()> {
        let named = Definition::from(Enum::Definition {
            name: Some(Identifier::new("fruit")?),
            values: vec![
                (Identifier::new("grape")?, None),
                (Identifier::new("cherry")?, None),
                (Identifier::new("lemon")?, None),
                (Identifier::new("kiwi")?, None),
            ]
            .try_into()?,
        })
        .to_string();
        assert_eq!(named, "enum fruit {grape, cherry, lemon, kiwi};");

        let specified_value = Definition::from(Enum::Definition {
            name: Some(Identifier::new("more_fruit")?),
            values: vec![
                (Identifier::new("banana")?, Some(Value::signed_integer(-17).into())),
                (Identifier::new("apple")?, None),
                (Identifier::new("blueberry")?, None),
                (Identifier::new("mango")?, None),
            ]
            .try_into()?,
        })
        .to_string();
        assert_eq!(
            specified_value,
            "enum more_fruit {banana = -17, apple, blueberry, mango};"
        );

        let specified_expression = Definition::from(Enum::Definition {
            name: Some(Identifier::new("yet_more_fruit")?),
            values: vec![
                (Identifier::new("kumquat")?, None),
                (Identifier::new("raspberry")?, None),
                (Identifier::new("peach")?, None),
                (
                    Identifier::new("plum")?,
                    Some(
                        BinaryOperator {
                            left: Variable::new("peach")?.into(),
                            operator: BinaryOperatorKind::Add,
                            right: Value::signed_integer(2).into(),
                        }
                        .into(),
                    ),
                ),
            ]
            .try_into()?,
        })
        .to_string();
        assert_eq!(
            specified_expression,
            "enum yet_more_fruit {kumquat, raspberry, peach, plum = peach + 2};"
        );

        Ok(())
    }

    #[test]
    fn tag_definition() -> anyhow::Result<()> {
        let tag = Definition::from(Enum::Tag {
            name: Identifier::new("fruit")?,
        })
        .to_string();
        assert_eq!(tag, "enum fruit;");

        Ok(())
    }

    #[test]
    fn declarations() -> anyhow::Result<()> {
        let inline = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Enum::Definition {
                name: Some(Identifier::new("fruit")?),
                values: vec![
                    (Identifier::new("banana")?, None),
                    (Identifier::new("apple")?, None),
                    (Identifier::new("blueberry")?, None),
                    (Identifier::new("mango")?, None),
                ]
                .try_into()?,
            }
            .into(),
            identifier: Identifier::new("my_fruit")?,
            initializer: None,
        })
        .to_string();
        assert_eq!(
            inline,
            "enum fruit {banana, apple, blueberry, mango} my_fruit;"
        );

        let tag = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Enum::Tag {
                name: Identifier::new("fruit")?,
            }
            .into(),
            identifier: Identifier::new("my_fruit")?,
            initializer: None,
        })
        .to_string();
        assert_eq!(tag, "enum fruit my_fruit;");

        Ok(())
    }

    #[test]
    fn initializers() -> anyhow::Result<()> {
        let generated = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Enum::Definition {
                name: Some(Identifier::new("fruit")?),
                values: vec![
                    (Identifier::new("banana")?, None),
                    (Identifier::new("apple")?, None),
                    (Identifier::new("blueberry")?, None),
                    (Identifier::new("mango")?, None),
                ]
                .try_into()?,
            }
            .into(),
            identifier: Identifier::new("my_fruit")?,
            initializer: Some(Variable::new("apple")?.into()),
        })
        .to_string();
        assert_eq!(
            generated,
            "enum fruit {banana, apple, blueberry, mango} my_fruit = apple;"
        );

        Ok(())
    }
}
