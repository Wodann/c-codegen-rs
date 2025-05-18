use pretty::Pretty;

use crate::{non_empty_vec::NonEmptyVec, pretty::impl_display_via_pretty, Identifier};

use super::member::Member;

#[derive(Clone)]
pub enum Union {
    Definition {
        name: Option<Identifier>,
        members: NonEmptyVec<Member>,
    },
    /// An incomplete union type, only useable as pointer type. Requires a complete definiton elsewhere.
    Tag { name: Identifier },
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Union
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let builder = allocator.text("union").append(allocator.space());

        match self {
            Union::Definition { name, members } => {
                let builder = if let Some(name) = name {
                    builder
                        .append(allocator.text(name.to_string()))
                        .append(allocator.space())
                } else {
                    builder
                };

                builder
                    .append(allocator.text("{"))
                    .append(allocator.hardline())
                    .append(
                        allocator.intersperse(
                            members.into_iter().map(|member| {
                                allocator.text("  ").append(member.pretty(allocator))
                            }),
                            allocator.hardline(),
                        ),
                    )
                    .append(allocator.hardline())
                    .append(allocator.text("}"))
            }
            Union::Tag { name } => builder.append(allocator.text(name.to_string())),
        }
    }
}

impl_display_via_pretty!(Union, 80);

#[cfg(test)]
mod tests {
    use core::f64;

    use crate::{
        r#type::{Definition, InitializerList},
        variable, ConcreteType, Statement, Value,
    };

    use super::*;

    #[test]
    fn complete_definitions() -> anyhow::Result<()> {
        let multi_line = Definition::from(Union::Definition {
            name: Some(Identifier::new("numbers")?),
            members: vec![
                Member {
                    ty: ConcreteType::int(),
                    name: Identifier::new("i")?,
                    bit_field_size: None,
                },
                Member {
                    ty: ConcreteType::float(),
                    name: Identifier::new("f")?,
                    bit_field_size: None,
                },
            ]
            .try_into()?,
        })
        .to_string();
        assert_eq!(
            multi_line,
            r#"union numbers {
  int i;
  float f;
};"#
        );

        Ok(())
    }

    #[test]
    fn incomplete_definition() -> anyhow::Result<()> {
        let generated = Definition::from(Union::Tag {
            name: Identifier::new("numbers")?,
        })
        .to_string();
        assert_eq!(generated, "union numbers;");

        Ok(())
    }

    #[test]
    fn declarations() -> anyhow::Result<()> {
        // Test inline declaration
        let inline = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Union::Definition {
                name: Some(Identifier::new("numbers")?),
                members: vec![
                    Member {
                        ty: ConcreteType::int(),
                        name: Identifier::new("i")?,
                        bit_field_size: None,
                    },
                    Member {
                        ty: ConcreteType::float(),
                        name: Identifier::new("f")?,
                        bit_field_size: None,
                    },
                ]
                .try_into()?,
            }
            .into(),
            identifier: Identifier::new("first_number")?,
            initializer: None,
        })
        .to_string();
        assert_eq!(
            inline,
            r#"union numbers {
  int i;
  float f;
} first_number;"#
        );

        let tag = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Union::Tag {
                name: Identifier::new("numbers")?,
            }
            .into(),
            identifier: Identifier::new("first_number")?,
            initializer: None,
        })
        .to_string();
        assert_eq!(tag, "union numbers first_number;");

        Ok(())
    }

    #[test]
    fn initializers() -> anyhow::Result<()> {
        let ordered = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Union::Tag {
                name: Identifier::new("numbers")?,
            }
            .into(),
            identifier: Identifier::new("first_number")?,
            initializer: Some(InitializerList::Ordered(vec![Value::int(5).into()]).into()),
        })
        .to_string();
        assert_eq!(ordered, "union numbers first_number = { 5 };");

        let named = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Union::Tag {
                name: Identifier::new("numbers")?,
            }
            .into(),
            identifier: Identifier::new("first_number")?,
            initializer: Some(
                InitializerList::Named(vec![(
                    Identifier::new("f")?,
                    Value::float(f64::consts::PI).into(),
                )])
                .into(),
            ),
        })
        .to_string();
        assert_eq!(
            named,
            "union numbers first_number = { .f = 3.141592653589793 };"
        );

        Ok(())
    }

    #[test]
    fn bit_fields() -> anyhow::Result<()> {
        let multi_line = Definition::from(Union::Definition {
            name: Some(Identifier::new("numbers")?),
            members: vec![
                Member {
                    ty: ConcreteType::unsigned_int(),
                    name: Identifier::new("ui")?,
                    bit_field_size: Some(2),
                },
                Member {
                    ty: ConcreteType::int(),
                    name: Identifier::new("i")?,
                    bit_field_size: Some(4),
                },
            ]
            .try_into()?,
        })
        .to_string();
        assert_eq!(
            multi_line,
            r#"union numbers {
  unsigned int ui : 2;
  int i : 4;
};"#
        );

        Ok(())
    }
}
