use pretty::Pretty;

use crate::{non_empty_vec::NonEmptyVec, pretty::impl_display_via_pretty, Identifier, Type};

#[derive(Clone)]
pub struct MemberGroup {
    pub ty: Type,
    pub members: NonEmptyVec<Member>,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for MemberGroup
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        allocator
            .text(self.ty.to_string())
            .append(allocator.space())
            .append(
                allocator.intersperse(
                    self.members
                        .into_iter()
                        .map(|member| member.pretty(allocator)),
                    allocator.text(",").append(allocator.space()),
                ),
            )
            .append(allocator.text(";"))
    }
}

#[derive(Clone)]
pub struct Member {
    pub name: Identifier,
    pub bit_field_size: Option<usize>,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Member
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let mut builder = allocator.text(self.name.to_string());

        if let Some(size) = self.bit_field_size {
            builder = builder
                .append(allocator.space())
                .append(allocator.text(":"))
                .append(allocator.space())
                .append(allocator.text(size.to_string()));
        }

        builder
    }
}

#[derive(Clone)]
pub enum Struct {
    Definition {
        name: Option<Identifier>,
        member_groups: NonEmptyVec<MemberGroup>,
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
        r#type::{Definition, InitializerList},
        variable, Value,
    };

    use super::*;

    #[test]
    fn complete_definitions() -> anyhow::Result<()> {
        let single_line = Definition::from(Struct::Definition {
            name: Some(Identifier::new("point")?),
            member_groups: vec![MemberGroup {
                ty: Type::int(),
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
            }]
            .try_into()?,
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
                MemberGroup {
                    ty: Type::int(),
                    members: vec![Member {
                        name: Identifier::new("x")?,
                        bit_field_size: None,
                    }]
                    .try_into()?,
                },
                MemberGroup {
                    ty: Type::int(),
                    members: vec![Member {
                        name: Identifier::new("y")?,
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
        let inline = variable::Declaration {
            storage_class: None,
            ty: Struct::Definition {
                name: Some(Identifier::new("point")?),
                member_groups: vec![
                    MemberGroup {
                        ty: Type::int(),
                        members: vec![Member {
                            name: Identifier::new("x")?,
                            bit_field_size: None,
                        }]
                        .try_into()?,
                    },
                    MemberGroup {
                        ty: Type::int(),
                        members: vec![Member {
                            name: Identifier::new("y")?,
                            bit_field_size: None,
                        }]
                        .try_into()?,
                    },
                ]
                .try_into()?,
            }
            .into(),
            variables: vec![
                (Identifier::new("first_point")?, None),
                (Identifier::new("second_point")?, None),
            ]
            .try_into()?,
        }
        .to_string();
        assert_eq!(
            inline,
            r#"struct point {
  int x;
  int y;
} first_point, second_point;"#
        );

        let incomplete = variable::Declaration {
            storage_class: None,
            ty: Struct::Tag {
                name: Identifier::new("point")?,
            }
            .into(),
            variables: vec![(Identifier::new("my_point")?, None)].try_into()?,
        }
        .to_string();
        assert_eq!(incomplete, "struct point my_point;");

        Ok(())
    }

    #[test]
    fn initializers() -> anyhow::Result<()> {
        let ordered = variable::Declaration {
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
        }
        .to_string();
        assert_eq!(ordered, "struct point first_point = { 5, 10 };");

        let named = variable::Declaration {
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
        }
        .to_string();
        assert_eq!(named, "struct point first_point = { .y = 10, .x = 5 };");

        let nested = variable::Declaration {
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
        }
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
            member_groups: vec![
                MemberGroup {
                    ty: Type::unsigned_int(),
                    members: vec![Member {
                        name: Identifier::new("suit")?,
                        bit_field_size: Some(2),
                    }]
                    .try_into()?,
                },
                MemberGroup {
                    ty: Type::unsigned_int(),
                    members: vec![Member {
                        name: Identifier::new("face_value")?,
                        bit_field_size: Some(4),
                    }]
                    .try_into()?,
                },
            ]
            .try_into()?,
        })
        .to_string();
        assert_eq!(
            single_line,
            r#"struct card {
  unsigned int suit : 2;
  unsigned int face_value : 4;
};"#
        );

        let multi_line = Definition::from(Struct::Definition {
            name: Some(Identifier::new("card")?),
            member_groups: vec![MemberGroup {
                ty: Type::unsigned_int(),
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
            }]
            .try_into()?,
        })
        .to_string();
        assert_eq!(
            multi_line,
            r#"struct card {
  unsigned int suit : 2, face_value : 4;
};"#
        );

        Ok(())
    }
}
