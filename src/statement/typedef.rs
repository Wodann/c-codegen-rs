use crate::{r#type::OpaqueType, Identifier};
use pretty::Pretty;

#[derive(Clone)]
pub struct Typedef {
    pub ty: OpaqueType,
    pub alias: Identifier,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Typedef
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let definition = match self.ty {
            OpaqueType::ConcreteType(concrete) => concrete.pretty_definition(self.alias, allocator),
            OpaqueType::Function(function) => function
                .pretty_signature_start(allocator)
                .append(allocator.text(self.alias))
                .append(function.pretty_signature_end(allocator)),
        };

        allocator
            .text("typedef")
            .append(allocator.space())
            .append(definition)
            .append(allocator.text(";"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        function::FunctionParameter,
        r#type::{member::Member, structure::Struct, Array, Function, Pointer},
        ConcreteType, Statement,
    };

    #[test]
    fn primitive() -> anyhow::Result<()> {
        let generated = Statement::from(Typedef {
            ty: ConcreteType::unsigned_char().into(),
            alias: Identifier::new("byte_type")?,
        })
        .to_string();
        assert_eq!(generated, "typedef unsigned char byte_type;");

        Ok(())
    }

    #[test]
    fn structure() -> anyhow::Result<()> {
        let typedef = Statement::from(Typedef {
            ty: Struct::Definition {
                name: Some(Identifier::new("fish")?),
                members: vec![
                    Member {
                        ty: ConcreteType::float(),
                        name: Identifier::new("weight")?,
                        bit_field_size: None,
                    },
                    Member {
                        ty: ConcreteType::float(),
                        name: Identifier::new("length")?,
                        bit_field_size: None,
                    },
                    Member {
                        ty: ConcreteType::float(),
                        name: Identifier::new("probability_of_being_caught")?,
                        bit_field_size: None,
                    },
                ],
            }
            .into(),
            alias: Identifier::new("fish_type")?,
        });
        assert_eq!(
            typedef.to_string(),
            r#"typedef struct fish {
  float weight;
  float length;
  float probability_of_being_caught;
} fish_type;"#
        );

        Ok(())
    }

    #[test]
    fn array() -> anyhow::Result<()> {
        let typedef = Statement::from(Typedef {
            ty: Array {
                element_type: Box::new(ConcreteType::Char),
                size: Some(5),
            }
            .into(),
            alias: Identifier::new("array_of_bytes")?,
        })
        .to_string();
        assert_eq!(typedef, "typedef char array_of_bytes[5];");

        Ok(())
    }

    #[test]
    fn array_of_function_pointers() -> anyhow::Result<()> {
        let typedef = Statement::from(Typedef {
            ty: Array {
                element_type: Box::new(
                    Pointer {
                        pointer_ty: Function {
                            parameters: vec![FunctionParameter {
                                ty: ConcreteType::int(),
                                name: Some(Identifier::new("x")?),
                            }],
                            return_ty: ConcreteType::Void,
                        }
                        .into(),
                        is_const: false,
                    }
                    .into(),
                ),
                size: None,
            }
            .into(),
            alias: Identifier::new("func_ptr_arr")?,
        })
        .to_string();
        assert_eq!(typedef, "typedef void (*func_ptr_arr[])(int x);");

        Ok(())
    }

    #[test]
    fn array_of_function_pointer_pointers() -> anyhow::Result<()> {
        let typedef = Statement::from(Typedef {
            ty: Array {
                element_type: Box::new(
                    Pointer {
                        pointer_ty: Pointer {
                            pointer_ty: Function {
                                parameters: vec![FunctionParameter {
                                    ty: ConcreteType::int(),
                                    name: Some(Identifier::new("x")?),
                                }],
                                return_ty: ConcreteType::Void,
                            }
                            .into(),
                            is_const: false,
                        }
                        .into(),
                        is_const: true,
                    }
                    .into(),
                ),
                size: None,
            }
            .into(),
            alias: Identifier::new("func_ptr_arr")?,
        })
        .to_string();
        assert_eq!(typedef, "typedef void (*const *func_ptr_arr[])(int x);");

        Ok(())
    }

    #[test]
    fn function() -> anyhow::Result<()> {
        let typedef = Statement::from(Typedef {
            ty: Function {
                parameters: vec![FunctionParameter {
                    ty: ConcreteType::int(),
                    name: Some(Identifier::new("x")?),
                }],
                return_ty: ConcreteType::Void,
            }
            .into(),
            alias: Identifier::new("func_type")?,
        })
        .to_string();
        assert_eq!(typedef, "typedef void (func_type)(int x);");

        Ok(())
    }

    #[test]
    fn function_pointer() -> anyhow::Result<()> {
        let typedef = Statement::from(Typedef {
            ty: Pointer {
                pointer_ty: Function {
                    parameters: vec![FunctionParameter {
                        ty: ConcreteType::int(),
                        name: Some(Identifier::new("x")?),
                    }],
                    return_ty: ConcreteType::Void,
                }
                .into(),
                is_const: false,
            }
            .into(),
            alias: Identifier::new("func_ptr")?,
        })
        .to_string();
        assert_eq!(typedef, "typedef void (*func_ptr)(int x);");

        Ok(())
    }
}
