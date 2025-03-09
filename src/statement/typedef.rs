use crate::{Identifier, Type};
use pretty::Pretty;

#[derive(Clone)]
pub struct Typedef {
    pub ty: Type,
    pub alias: Identifier,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Typedef
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let base_type = self.ty.base_type();
        let dimensions = self.ty.pretty_dimensions(allocator);

        allocator
            .text("typedef")
            .append(allocator.space())
            .append(base_type.pretty(allocator))
            .append(allocator.space())
            .append(allocator.text(self.alias).append(dimensions))
            .append(allocator.text(";"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        r#type::{
            member::{self, Member},
            structure::Struct,
            Array,
        },
        Statement,
    };

    #[test]
    fn primitive() -> anyhow::Result<()> {
        let generated = Statement::from(Typedef {
            ty: Type::unsigned_char(),
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
                        ty: Type::float(),
                        members: vec![Member {
                            name: Identifier::new("weight")?,
                            bit_field_size: None,
                        }]
                        .try_into()?,
                    },
                    member::Group {
                        ty: Type::float(),
                        members: vec![member::Member {
                            name: Identifier::new("length")?,
                            bit_field_size: None,
                        }]
                        .try_into()?,
                    },
                    member::Group {
                        ty: Type::float(),
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
                element_type: Box::new(Type::Char),
                size: Some(5),
            }
            .into(),
            alias: Identifier::new("array_of_bytes")?,
        })
        .to_string();
        assert_eq!(typedef, "typedef char array_of_bytes[5];");

        Ok(())
    }
}
