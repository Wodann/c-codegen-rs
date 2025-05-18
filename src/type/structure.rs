use pretty::Pretty;

use crate::{pretty::impl_display_via_pretty, Identifier};

use super::member::Member;

#[derive(Clone)]
pub enum Struct {
    Definition {
        name: Option<Identifier>,
        members: Vec<Member>,
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
            Struct::Definition { name, members } => {
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
            Struct::Tag { name } => builder.append(allocator.text(name.to_string())),
        }
    }
}

impl_display_via_pretty!(Struct, 80);

#[cfg(test)]
mod tests {
    use crate::{
        function::FunctionParameter,
        r#type::{Definition, Function, InitializerList, Pointer},
        variable, ConcreteType, Statement, Value,
    };

    use super::*;

    #[test]
    fn complete_definitions() -> anyhow::Result<()> {
        let multi_line = Definition::from(Struct::Definition {
            name: Some(Identifier::new("point")?),
            members: vec![
                Member {
                    ty: ConcreteType::int(),
                    name: Identifier::new("x")?,
                    bit_field_size: None,
                },
                Member {
                    ty: ConcreteType::int(),
                    name: Identifier::new("y")?,
                    bit_field_size: None,
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
                members: vec![
                    Member {
                        ty: ConcreteType::int(),
                        name: Identifier::new("x")?,
                        bit_field_size: None,
                    },
                    Member {
                        ty: ConcreteType::int(),
                        name: Identifier::new("y")?,
                        bit_field_size: None,
                    },
                ],
            }
            .into(),
            identifier: Identifier::new("first_point")?,
            initializer: None,
        })
        .to_string();
        assert_eq!(
            inline,
            r#"struct point {
  int x;
  int y;
} first_point;"#
        );

        let tag = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Struct::Tag {
                name: Identifier::new("point")?,
            }
            .into(),
            identifier: Identifier::new("my_point")?,
            initializer: None,
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
            identifier: Identifier::new("first_point")?,
            initializer: Some(
                InitializerList::Ordered(vec![Value::int(5).into(), Value::int(10).into()]).into(),
            ),
        })
        .to_string();
        assert_eq!(ordered, "struct point first_point = { 5, 10 };");

        let named = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Struct::Tag {
                name: Identifier::new("point")?,
            }
            .into(),
            identifier: Identifier::new("first_point")?,
            initializer: Some(
                InitializerList::Named(vec![
                    (Identifier::new("y")?, Value::int(10).into()),
                    (Identifier::new("x")?, Value::int(5).into()),
                ])
                .into(),
            ),
        })
        .to_string();
        assert_eq!(named, "struct point first_point = { .y = 10, .x = 5 };");

        let nested = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Struct::Tag {
                name: Identifier::new("rectangle")?,
            }
            .into(),
            identifier: Identifier::new("my_rectangle")?,
            initializer: Some(
                InitializerList::Ordered(vec![
                    InitializerList::Ordered(vec![Value::int(0).into(), Value::int(5).into()])
                        .into(),
                    InitializerList::Ordered(vec![Value::int(10).into(), Value::int(0).into()])
                        .into(),
                ])
                .into(),
            ),
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
        let multi_line = Definition::from(Struct::Definition {
            name: Some(Identifier::new("card")?),
            members: vec![
                Member {
                    ty: ConcreteType::unsigned_int(),
                    name: Identifier::new("suit")?,
                    bit_field_size: Some(2),
                },
                Member {
                    ty: ConcreteType::unsigned_int(),
                    name: Identifier::new("face_value")?,
                    bit_field_size: Some(4),
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
    fn function_pointer_member() -> anyhow::Result<()> {
        let generated = Definition::from(Struct::Definition {
            name: Some(Identifier::new("with_pointers")?),
            members: vec![
                Member {
                    ty: Pointer {
                        pointer_ty: Function {
                            parameters: vec![
                                FunctionParameter {
                                    ty: ConcreteType::int(),
                                    name: None,
                                },
                                FunctionParameter {
                                    ty: ConcreteType::int(),
                                    name: None,
                                },
                            ],
                            return_ty: ConcreteType::int(),
                        }
                        .into(),
                        is_const: false,
                    }
                    .into(),
                    name: Identifier::new("mutable")?,
                    bit_field_size: None,
                },
                Member {
                    ty: Pointer {
                        pointer_ty: Function {
                            parameters: vec![],
                            return_ty: ConcreteType::Void,
                        }
                        .into(),
                        is_const: true,
                    }
                    .into(),
                    name: Identifier::new("immutable")?,
                    bit_field_size: None,
                },
            ],
        })
        .to_string();
        assert_eq!(
            generated,
            r#"struct with_pointers {
  int (*mutable)(int, int);
  void (*const immutable)();
};"#
        );

        Ok(())
    }

    #[test]
    fn pointer_member() -> anyhow::Result<()> {
        let generated = Definition::from(Struct::Definition {
            name: Some(Identifier::new("with_pointers")?),
            members: vec![
                Member {
                    ty: Pointer {
                        pointer_ty: ConcreteType::int().into(),
                        is_const: false,
                    }
                    .into(),
                    name: Identifier::new("mutable")?,
                    bit_field_size: None,
                },
                Member {
                    ty: Pointer {
                        pointer_ty: ConcreteType::Void.into(),
                        is_const: true,
                    }
                    .into(),
                    name: Identifier::new("immutable")?,
                    bit_field_size: None,
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
