use pretty::Pretty;

use crate::{pretty::impl_display_via_pretty, Identifier};

use super::member;

#[derive(Clone)]
pub enum Struct {
    Definition {
        name: Option<Identifier>,
        member_groups: Vec<member::Group>,
    },
    /// An incomplete structure type, only useable as pointer type. Requires a complete definiton elsewhere.
    Tag { name: Identifier },
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Struct
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let builder = allocator.text("struct").append(allocator.space());

        match self {
            Struct::Definition {
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
            Struct::Tag { name } => builder.append(allocator.text(name.to_string())),
        }
    }
}

impl_display_via_pretty!(Struct, 80);

#[cfg(test)]
mod tests {

    use crate::{
        r#type::{Definition, InitializerList, Pointer},
        variable, ConcreteType, Statement, Value,
    };

    use super::{member::Member, *};

    #[test]
    fn complete_definitions() -> anyhow::Result<()> {
        let single_line = Definition::from(Struct::Definition {
            name: Some(Identifier::new("point")?),
            member_groups: vec![member::Group {
                ty: ConcreteType::int(),
                members: vec![
                    Member {
                        name: Identifier::new("x")?,
                        bit_field_size: None,
                    },
                    Member {
                        name: Identifier::new("y")?,
                        bit_field_size: None,
                    },
                ]
                .try_into()?,
            }],
        })
        .to_string();
        assert_eq!(
            single_line,
            r#"struct point {
  int x, y;
};"#
        );

        let multi_line = Definition::from(Struct::Definition {
            name: Some(Identifier::new("point")?),
            member_groups: vec![
                member::Group {
                    ty: ConcreteType::int(),
                    members: vec![Member {
                        name: Identifier::new("x")?,
                        bit_field_size: None,
                    }]
                    .try_into()?,
                },
                member::Group {
                    ty: ConcreteType::int(),
                    members: vec![Member {
                        name: Identifier::new("y")?,
                        bit_field_size: None,
                    }]
                    .try_into()?,
                },
            ],
        })
        .to_string();
        assert_eq!(
            multi_line,
            r#"struct point {
  int x;
  int y;
};"#
        );

        Ok(())
    }

    #[test]
    fn incomplete_definition() -> anyhow::Result<()> {
        let generated = Definition::from(Struct::Tag {
            name: Identifier::new("point")?,
        })
        .to_string();
        assert_eq!(generated, "struct point;");

        Ok(())
    }

    #[test]
    fn declarations() -> anyhow::Result<()> {
        // Test inline declaration
        let inline = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Struct::Definition {
                name: Some(Identifier::new("point")?),
                member_groups: vec![
                    member::Group {
                        ty: ConcreteType::int(),
                        members: vec![Member {
                            name: Identifier::new("x")?,
                            bit_field_size: None,
                        }]
                        .try_into()?,
                    },
                    member::Group {
                        ty: ConcreteType::int(),
                        members: vec![Member {
                            name: Identifier::new("y")?,
                            bit_field_size: None,
                        }]
                        .try_into()?,
                    },
                ],
            }
            .into(),
            variables: vec![
                (Identifier::new("first_point")?, None),
                (Identifier::new("second_point")?, None),
            ]
            .try_into()?,
        })
        .to_string();
        assert_eq!(
            inline,
            r#"struct point {
  int x;
  int y;
} first_point, second_point;"#
        );

        let tag = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Struct::Tag {
                name: Identifier::new("point")?,
            }
            .into(),
            variables: vec![(Identifier::new("my_point")?, None)].try_into()?,
        })
        .to_string();
        assert_eq!(tag, "struct point my_point;");

        Ok(())
    }

    #[test]
    fn initializers() -> anyhow::Result<()> {
        let ordered = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Struct::Tag {
                name: Identifier::new("point")?,
            }
            .into(),
            variables: vec![(
                Identifier::new("first_point")?,
                Some(
                    InitializerList::Ordered(vec![Value::int(5).into(), Value::int(10).into()])
                        .into(),
                ),
            )]
            .try_into()?,
        })
        .to_string();
        assert_eq!(ordered, "struct point first_point = { 5, 10 };");

        let named = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Struct::Tag {
                name: Identifier::new("point")?,
            }
            .into(),
            variables: vec![(
                Identifier::new("first_point")?,
                Some(
                    InitializerList::Named(vec![
                        (Identifier::new("y")?, Value::int(10).into()),
                        (Identifier::new("x")?, Value::int(5).into()),
                    ])
                    .into(),
                ),
            )]
            .try_into()?,
        })
        .to_string();
        assert_eq!(named, "struct point first_point = { .y = 10, .x = 5 };");

        let nested = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Struct::Tag {
                name: Identifier::new("rectangle")?,
            }
            .into(),
            variables: vec![(
                Identifier::new("my_rectangle")?,
                Some(
                    InitializerList::Ordered(vec![
                        InitializerList::Ordered(vec![Value::int(0).into(), Value::int(5).into()])
                            .into(),
                        InitializerList::Ordered(vec![Value::int(10).into(), Value::int(0).into()])
                            .into(),
                    ])
                    .into(),
                ),
            )]
            .try_into()?,
        })
        .to_string();
        assert_eq!(
            nested,
            "struct rectangle my_rectangle = { { 0, 5 }, { 10, 0 } };"
        );

        Ok(())
    }

    #[test]
    fn bit_fields() -> anyhow::Result<()> {
        let single_line = Definition::from(Struct::Definition {
            name: Some(Identifier::new("card")?),
            member_groups: vec![member::Group {
                ty: ConcreteType::unsigned_int(),
                members: vec![
                    Member {
                        name: Identifier::new("suit")?,
                        bit_field_size: Some(2),
                    },
                    Member {
                        name: Identifier::new("face_value")?,
                        bit_field_size: Some(4),
                    },
                ]
                .try_into()?,
            }],
        })
        .to_string();
        assert_eq!(
            single_line,
            r#"struct card {
  unsigned int suit : 2, face_value : 4;
};"#
        );

        let multi_line = Definition::from(Struct::Definition {
            name: Some(Identifier::new("card")?),
            member_groups: vec![
                member::Group {
                    ty: ConcreteType::unsigned_int(),
                    members: vec![Member {
                        name: Identifier::new("suit")?,
                        bit_field_size: Some(2),
                    }]
                    .try_into()?,
                },
                member::Group {
                    ty: ConcreteType::unsigned_int(),
                    members: vec![Member {
                        name: Identifier::new("face_value")?,
                        bit_field_size: Some(4),
                    }]
                    .try_into()?,
                },
            ],
        })
        .to_string();
        assert_eq!(
            multi_line,
            r#"struct card {
  unsigned int suit : 2;
  unsigned int face_value : 4;
};"#
        );

        Ok(())
    }

    #[test]
    fn pointer_member() -> anyhow::Result<()> {
        let generated = Definition::from(Struct::Definition {
            name: Some(Identifier::new("with_pointers")?),
            member_groups: vec![
                member::Group {
                    ty: Pointer {
                        pointer_ty: ConcreteType::int().into(),
                        is_const: false,
                    }
                    .into(),
                    members: vec![Member {
                        name: Identifier::new("mutable")?,
                        bit_field_size: None,
                    }]
                    .try_into()?,
                },
                member::Group {
                    ty: Pointer {
                        pointer_ty: ConcreteType::Void.into(),
                        is_const: true,
                    }
                    .into(),
                    members: vec![Member {
                        name: Identifier::new("immutable")?,
                        bit_field_size: None,
                    }]
                    .try_into()?,
                },
            ],
        })
        .to_string();
        assert_eq!(
            generated,
            r#"struct with_pointers {
  int *mutable;
  void *const immutable;
};"#
        );

        Ok(())
    }
}
