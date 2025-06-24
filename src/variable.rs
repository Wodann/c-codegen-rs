use pretty::Pretty;

use crate::{pretty::impl_display_via_pretty, ConcreteType, Expression, Identifier, StorageClass};

pub type Variable = Identifier;

/// Variable declaration
///
/// Unsupported C language features:
///
/// ```c
/// int test = 1, test2[2] = {1, 2};
/// ```
#[derive(Clone, Debug)]
pub struct Declaration {
    pub storage_class: Option<StorageClass>,
    pub ty: ConcreteType,
    pub identifier: Identifier,
    pub initializer: Option<Expression>,
}

// Implement Pretty for Declaration
impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Declaration
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let builder = if let Some(storage_class) = self.storage_class {
            allocator
                .text(storage_class.to_string())
                .append(allocator.space())
        } else {
            allocator.nil()
        };

        let builder = builder.append(self.ty.pretty_definition(self.identifier, allocator));

        if let Some(initializer) = self.initializer {
            builder
                .append(allocator.space())
                .append(allocator.text("="))
                .append(allocator.space())
                .append(initializer.pretty(allocator))
        } else {
            builder
        }
    }
}

impl_display_via_pretty!(Declaration, 80);

#[cfg(test)]
mod tests {
    use crate::{
        function::FunctionParameter,
        r#type::{Function, Pointer},
        Statement, Value,
    };

    use super::*;

    #[test]
    fn const_pointer() -> anyhow::Result<()> {
        let generated = Statement::from(Declaration {
            storage_class: None,
            ty: Pointer {
                pointer_ty: ConcreteType::int().into(),
                is_const: true,
            }
            .into(),
            identifier: Identifier::new("x")?,
            initializer: None,
        })
        .to_string();
        assert_eq!(generated, "int *const x;");

        Ok(())
    }

    #[test]
    fn function_pointer() -> anyhow::Result<()> {
        let immutable = Statement::from(Declaration {
            storage_class: None,
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
                is_const: true,
            }
            .into(),
            identifier: Identifier::new("immutable")?,
            initializer: None,
        })
        .to_string();
        assert_eq!(immutable, "int (*const immutable)(int, int);");

        let mutable = Statement::from(Declaration {
            storage_class: None,
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
            identifier: Identifier::new("mutable")?,
            initializer: None,
        })
        .to_string();
        assert_eq!(mutable, "int (*mutable)(int, int);");

        Ok(())
    }

    #[test]
    fn initializer() -> anyhow::Result<()> {
        let multiple = Statement::from(Declaration {
            storage_class: None,
            ty: ConcreteType::int(),
            identifier: Identifier::new("x")?,
            initializer: Some(Value::signed_integer(5).into()),
        })
        .to_string();
        assert_eq!(multiple, "int x = 5;");

        Ok(())
    }

    #[test]
    fn pointer() -> anyhow::Result<()> {
        let generated = Statement::from(Declaration {
            storage_class: None,
            ty: Pointer {
                pointer_ty: ConcreteType::int().into(),
                is_const: false,
            }
            .into(),
            identifier: Identifier::new("x")?,
            initializer: None,
        })
        .to_string();
        assert_eq!(generated, "int *x;");

        Ok(())
    }
}
