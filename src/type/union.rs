use pretty::Pretty;

use crate::{non_empty_vec::NonEmptyVec, pretty::impl_display_via_pretty, Identifier};

use super::member;

#[derive(Clone)]
pub enum Union {
    Definition {
        name: Option<Identifier>,
        member_groups: NonEmptyVec<member::Group>,
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
            Union::Definition {
                name,
                member_groups,
            } => {
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
                            member_groups.into_iter().map(|member| {
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
    use crate::{
        r#type::{Definition, InitializerList},
        variable, Type, Value,
    };

    use super::{member::Member, *};

    #[test]
    fn complete_definitions() -> anyhow::Result<()> {
        let single_line = Definition::from(Union::Definition {
            name: Some(Identifier::new("numbers")?),
            member_groups: vec![member::Group {
                ty: Type::int(),
                members: vec![
                    Member {
                        name: Identifier::new("i")?,
                        bit_field_size: None,
                    },
                    Member {
                        name: Identifier::new("j")?,
                        bit_field_size: None,
                    },
                ]
                .try_into()?,
            }]
            .try_into()?,
        })
        .to_string();
        assert_eq!(
            single_line,
            r#"union numbers {
  int i, j;
};"#
        );

        let multi_line = Definition::from(Union::Definition {
            name: Some(Identifier::new("numbers")?),
            member_groups: vec![
                member::Group {
                    ty: Type::int(),
                    members: vec![Member {
                        name: Identifier::new("i")?,
                        bit_field_size: None,
                    }]
                    .try_into()?,
                },
                member::Group {
                    ty: Type::float(),
                    members: vec![Member {
                        name: Identifier::new("f")?,
                        bit_field_size: None,
                    }]
                    .try_into()?,
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
        let inline = variable::Declaration {
            storage_class: None,
            ty: Union::Definition {
                name: Some(Identifier::new("numbers")?),
                member_groups: vec![
                    member::Group {
                        ty: Type::int(),
                        members: vec![Member {
                            name: Identifier::new("i")?,
                            bit_field_size: None,
                        }]
                        .try_into()?,
                    },
                    member::Group {
                        ty: Type::float(),
                        members: vec![Member {
                            name: Identifier::new("f")?,
                            bit_field_size: None,
                        }]
                        .try_into()?,
                    },
                ]
                .try_into()?,
            }
            .into(),
            variables: vec![
                (Identifier::new("first_number")?, None),
                (Identifier::new("second_number")?, None),
            ]
            .try_into()?,
        }
        .to_string();
        assert_eq!(
            inline,
            r#"union numbers {
  int i;
  float f;
} first_number, second_number;"#
        );

        let tag = variable::Declaration {
            storage_class: None,
            ty: Union::Tag {
                name: Identifier::new("numbers")?,
            }
            .into(),
            variables: vec![
                (Identifier::new("first_number")?, None),
                (Identifier::new("second_number")?, None),
            ]
            .try_into()?,
        }
        .to_string();
        assert_eq!(tag, "union numbers first_number, second_number;");

        Ok(())
    }

    #[test]
    fn initializers() -> anyhow::Result<()> {
        let ordered = variable::Declaration {
            storage_class: None,
            ty: Union::Tag {
                name: Identifier::new("numbers")?,
            }
            .into(),
            variables: vec![(
                Identifier::new("first_number")?,
                Some(InitializerList::Ordered(vec![Value::int(5).into()]).into()),
            )]
            .try_into()?,
        }
        .to_string();
        assert_eq!(ordered, "union numbers first_number = { 5 };");

        let named = variable::Declaration {
            storage_class: None,
            ty: Union::Tag {
                name: Identifier::new("numbers")?,
            }
            .into(),
            variables: vec![(
                Identifier::new("first_number")?,
                Some(
                    InitializerList::Named(vec![(
                        Identifier::new("f")?,
                        Value::float(3.14159).into(),
                    )])
                    .into(),
                ),
            )]
            .try_into()?,
        }
        .to_string();
        assert_eq!(named, "union numbers first_number = { .f = 3.14159 };");

        Ok(())
    }

    #[test]
    fn bit_fields() -> anyhow::Result<()> {
        let single_line = Definition::from(Union::Definition {
            name: Some(Identifier::new("numbers")?),
            member_groups: vec![member::Group {
                ty: Type::unsigned_int(),
                members: vec![
                    Member {
                        name: Identifier::new("i2")?,
                        bit_field_size: Some(2),
                    },
                    Member {
                        name: Identifier::new("i4")?,
                        bit_field_size: Some(4),
                    },
                ]
                .try_into()?,
            }]
            .try_into()?,
        })
        .to_string();
        assert_eq!(
            single_line,
            r#"union numbers {
  unsigned int i2 : 2, i4 : 4;
};"#
        );

        let multi_line = Definition::from(Union::Definition {
            name: Some(Identifier::new("numbers")?),
            member_groups: vec![
                member::Group {
                    ty: Type::unsigned_int(),
                    members: vec![Member {
                        name: Identifier::new("ui")?,
                        bit_field_size: Some(2),
                    }]
                    .try_into()?,
                },
                member::Group {
                    ty: Type::int(),
                    members: vec![Member {
                        name: Identifier::new("i")?,
                        bit_field_size: Some(4),
                    }]
                    .try_into()?,
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
