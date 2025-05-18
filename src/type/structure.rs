use pretty::Pretty;

use crate::{pretty::impl_display_via_pretty, Identifier};

use super::member::Member;

/// An incomplete structure type, only useable in a declaration and as pointer type. Requires a complete definiton elsewhere.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Declaration {
    pub name: Identifier,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Declaration
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        allocator
            .text("struct")
            .append(allocator.space())
            .append(allocator.text(self.name.to_string()))
    }
}

#[derive(Clone, Debug)]
pub struct Definition {
    pub name: Option<Identifier>,
    pub members: Vec<Member>,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Definition
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let builder = allocator.text("struct").append(allocator.space());

        let builder = if let Some(name) = self.name {
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
                    self.members
                        .into_iter()
                        .map(|member| allocator.text("  ").append(member.pretty(allocator))),
                    allocator.hardline(),
                ),
            )
            .append(allocator.hardline())
            .append(allocator.text("}"))
    }
}

impl_display_via_pretty!(Definition, 80);

#[cfg(test)]
mod tests {
    use crate::{
        function::FunctionParameter,
        r#type::{self, Function, InitializerList, Pointer},
        variable, IncompleteType, Statement, Value,
    };

    use super::*;

    #[test]
    fn complete_definitions() -> anyhow::Result<()> {
        let multi_line = r#type::Definition::from(Definition {
            name: Some(Identifier::new("point")?),
            members: vec![
                Member {
                    ty: IncompleteType::int(),
                    name: Identifier::new("x")?,
                    bit_field_size: None,
                },
                Member {
                    ty: IncompleteType::int(),
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
        let generated = r#type::Declaration::from(Declaration {
            name: Identifier::new("point")?,
        })
        .to_string();
        assert_eq!(generated, "struct point");

        Ok(())
    }

    #[test]
    fn declarations() -> anyhow::Result<()> {
        // Test inline declaration
        let inline = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Definition {
                name: Some(Identifier::new("point")?),
                members: vec![
                    Member {
                        ty: IncompleteType::int(),
                        name: Identifier::new("x")?,
                        bit_field_size: None,
                    },
                    Member {
                        ty: IncompleteType::int(),
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
            ty: Declaration {
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
            ty: Declaration {
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
            ty: Declaration {
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
            ty: Declaration {
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
        let multi_line = r#type::Definition::from(Definition {
            name: Some(Identifier::new("card")?),
            members: vec![
                Member {
                    ty: IncompleteType::unsigned_int(),
                    name: Identifier::new("suit")?,
                    bit_field_size: Some(2),
                },
                Member {
                    ty: IncompleteType::unsigned_int(),
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
        let generated = r#type::Definition::from(Definition {
            name: Some(Identifier::new("with_pointers")?),
            members: vec![
                Member {
                    ty: Pointer {
                        pointer_ty: Function {
                            parameters: vec![
                                FunctionParameter {
                                    ty: IncompleteType::int(),
                                    name: None,
                                },
                                FunctionParameter {
                                    ty: IncompleteType::int(),
                                    name: None,
                                },
                            ],
                            return_ty: IncompleteType::int(),
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
                            return_ty: IncompleteType::Void,
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
        let generated = r#type::Definition::from(Definition {
            name: Some(Identifier::new("with_pointers")?),
            members: vec![
                Member {
                    ty: Pointer {
                        pointer_ty: IncompleteType::int().into(),
                        is_const: false,
                    }
                    .into(),
                    name: Identifier::new("mutable")?,
                    bit_field_size: None,
                },
                Member {
                    ty: Pointer {
                        pointer_ty: IncompleteType::Void.into(),
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
