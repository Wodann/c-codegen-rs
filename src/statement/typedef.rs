use crate::{r#type::OpaqueType, ConcreteType, Identifier};
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
        let builder = allocator.text("typedef").append(allocator.space());

        let alias = if let OpaqueType::ConcreteType(ConcreteType::Array(array)) = self.ty {
            let dimensions = array.pretty_dimensions(allocator);
            allocator.text(self.alias).append(dimensions)
        } else {
            allocator.text(self.alias)
        };

        let builder = if let OpaqueType::Function(function) = self.ty.base_type() {
            let return_type = function.pretty_return_type(allocator);
            let parameters = function.pretty_parameters(allocator);

            let mut builder = builder
                .append(return_type)
                .append(allocator.space())
                .append(allocator.text("("));

            if let ConcreteType::Pointer(pointer) = self.ty {
                let mut needs_space = false;
                for is_const in pointer.flatten_pointers() {
                    if needs_space {
                        builder = builder.append(allocator.space());
                    }

                    builder = builder.append(allocator.text("*"));

                    if is_const {
                        builder = builder.append(allocator.space().append(allocator.text("const")));
                    }

                    needs_space = true;
                }

                builder = builder.append(allocator.space());
            }

            builder
                .append(allocator.text(self.alias))
                .append(allocator.text(")"))
                .append(parameters)
        } else if let OpaqueType::ConcreteType(ConcreteType::Array(array)) = self.ty {
            let dimensions = self.ty.pretty_dimensions(allocator);

            builder
                .append(base_type.pretty(allocator))
                .append(allocator.space())
                .append(allocator.text(self.alias))
                .append(dimensions)
        } else {
            builder
                .append(self.ty.pretty(allocator))
                .append(allocator.space())
                .append(allocator.text(self.alias))
        };

        builder.append(allocator.text(";"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        function::FunctionParameter,
        r#type::{
            member::{self, Member},
            structure::Struct,
            Array, Function, Pointer,
        },
        Statement,
    };

    #[test]
    fn primitive() -> anyhow::Result<()> {
        let generated = Statement::from(Typedef {
            ty: ConcreteType::unsigned_char(),
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
                member_groups: vec![
                    member::Group {
                        ty: ConcreteType::float(),
                        members: vec![Member {
                            name: Identifier::new("weight")?,
                            bit_field_size: None,
                        }]
                        .try_into()?,
                    },
                    member::Group {
                        ty: ConcreteType::float(),
                        members: vec![member::Member {
                            name: Identifier::new("length")?,
                            bit_field_size: None,
                        }]
                        .try_into()?,
                    },
                    member::Group {
                        ty: ConcreteType::float(),
                        members: vec![member::Member {
                            name: Identifier::new("probability_of_being_caught")?,
                            bit_field_size: None,
                        }]
                        .try_into()?,
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
