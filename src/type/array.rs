use pretty::Pretty;

use crate::{pretty::impl_display_via_pretty, ConcreteType};

use super::OpaqueType;

/// Represents a C array type with its base type and size
#[derive(Clone, Debug)]
pub struct Array {
    /// The base type of the array elements
    pub element_type: Box<ConcreteType>,
    /// The size of the array (optional for flexible arrays)
    pub size: Option<usize>,
}

impl Array {
    /// Returns the fundamental type of the array, after stripping away all type constructors (like pointers and arrays).
    ///
    /// # Examples
    ///
    /// - For `int[3][4]`, it returns `int`.
    /// - For `void (*)(int, int)`, it returns `void (int, int)`.
    pub fn base_type(&self) -> OpaqueType {
        match self.element_type.as_ref() {
            ConcreteType::Array(array) => array.base_type(),
            ConcreteType::Pointer(pointer) => pointer.base_type(),
            ty => OpaqueType::ConcreteType(ty.clone()),
        }
    }

    /// Returns the innermost element type of the array.
    /// This is useful for determining the type of elements in a multi-dimensional array.
    /// For example, for `int[3][4]`, it returns `int`.
    pub fn innermost_element_type(&self) -> ConcreteType {
        match self.element_type.as_ref() {
            ConcreteType::Array(array) => array.innermost_element_type(),
            ty => ty.clone(),
        }
    }

    pub fn dimensions(&self) -> Vec<Option<usize>> {
        let mut dimensions = vec![self.size];

        if let ConcreteType::Array(array) = self.element_type.as_ref() {
            dimensions.extend(array.dimensions());
        }

        dimensions
    }

    pub(crate) fn pretty_dimensions<'a, AllocatorT, AnnotationT>(
        &self,
        allocator: &'a AllocatorT,
    ) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT>
    where
        AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
        AllocatorT::Doc: Clone,
        AnnotationT: Clone + 'a,
    {
        self.dimensions()
            .into_iter()
            .fold(allocator.nil(), |builder, dimension| {
                let dimension = match dimension {
                    Some(size) => allocator.text(size.to_string()),
                    None => allocator.nil(),
                };

                builder.append(dimension.brackets())
            })
    }
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Array
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let innermost_element = self.innermost_element_type().pretty(allocator);
        let dimensions = self.pretty_dimensions(allocator);

        innermost_element.append(dimensions)
    }
}

impl_display_via_pretty!(Array, 80);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        operator::{ArraySubscript, Assignment},
        r#type::{member::MemberAccess, structure::Struct, union::Union, InitializerList},
        variable, Identifier, Statement, Value, Variable,
    };

    #[test]
    fn fixed_width() -> anyhow::Result<()> {
        let definition = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Array {
                element_type: Box::new(ConcreteType::int()),
                size: Some(10),
            }
            .into(),
            identifier: Identifier::new("my_array")?,
            initializer: None,
        });
        assert_eq!(definition.to_string(), "int my_array[10];");

        let initialization = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Array {
                element_type: Box::new(ConcreteType::int()),
                size: Some(10),
            }
            .into(),
            identifier: Identifier::new("my_array")?,
            initializer: Some(
                InitializerList::Ordered(vec![
                    Value::signed_integer(0).into(),
                    Value::signed_integer(1).into(),
                    Value::signed_integer(2).into(),
                    Value::signed_integer(3).into(),
                    Value::signed_integer(4).into(),
                ])
                .into(),
            ),
        });
        assert_eq!(
            initialization.to_string(),
            "int my_array[10] = { 0, 1, 2, 3, 4 };"
        );

        Ok(())
    }

    #[test]
    fn flex_width() -> anyhow::Result<()> {
        let definition = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Array {
                element_type: Box::new(ConcreteType::int()),
                size: None,
            }
            .into(),
            identifier: Identifier::new("flex")?,
            initializer: None,
        });
        assert_eq!(definition.to_string(), "int flex[];");

        let initialization = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Array {
                element_type: Box::new(ConcreteType::int()),
                size: None,
            }
            .into(),
            identifier: Identifier::new("flex")?,
            initializer: Some(
                InitializerList::Ordered(vec![
                    Value::signed_integer(0).into(),
                    Value::signed_integer(1).into(),
                    Value::signed_integer(2).into(),
                    Value::signed_integer(3).into(),
                    Value::signed_integer(4).into(),
                ])
                .into(),
            ),
        });
        assert_eq!(
            initialization.to_string(),
            "int flex[] = { 0, 1, 2, 3, 4 };"
        );

        Ok(())
    }

    #[test]
    fn two_dimensional() -> anyhow::Result<()> {
        let inner_array = Array {
            element_type: Box::new(ConcreteType::int()),
            size: Some(5),
        };

        let outer_array = Array {
            element_type: Box::new(inner_array.into()),
            size: Some(2),
        };

        let definition = Statement::from(variable::Declaration {
            storage_class: None,
            ty: outer_array.clone().into(),
            identifier: Identifier::new("two_dimensions")?,
            initializer: None,
        });
        assert_eq!(definition.to_string(), "int two_dimensions[2][5];");

        let declaration = Statement::from(variable::Declaration {
            storage_class: None,
            ty: outer_array.into(),
            identifier: Identifier::new("two_dimensions")?,
            initializer: Some(
                InitializerList::Ordered(vec![
                    InitializerList::Ordered(vec![
                        Value::signed_integer(1).into(),
                        Value::signed_integer(2).into(),
                        Value::signed_integer(3).into(),
                        Value::signed_integer(4).into(),
                        Value::signed_integer(5).into(),
                    ])
                    .into(),
                    InitializerList::Ordered(vec![
                        Value::signed_integer(6).into(),
                        Value::signed_integer(7).into(),
                        Value::signed_integer(8).into(),
                        Value::signed_integer(9).into(),
                        Value::signed_integer(10).into(),
                    ])
                    .into(),
                ])
                .into(),
            ),
        });
        assert_eq!(
            declaration.to_string(),
            "int two_dimensions[2][5] = { { 1, 2, 3, 4, 5 }, { 6, 7, 8, 9, 10 } };"
        );

        Ok(())
    }

    #[test]
    fn three_dimensional() -> anyhow::Result<()> {
        let inner_array = Array {
            element_type: Box::new(ConcreteType::int()),
            size: Some(4),
        };

        let middle_array = Array {
            element_type: Box::new(inner_array.into()),
            size: Some(3),
        };

        let outer_array = Array {
            element_type: Box::new(middle_array.into()),
            size: Some(2),
        };

        let definition = Statement::from(variable::Declaration {
            storage_class: None,
            ty: outer_array.clone().into(),
            identifier: Identifier::new("three_dimensional")?,
            initializer: None,
        });
        assert_eq!(definition.to_string(), "int three_dimensional[2][3][4];");

        let declaration = Statement::from(variable::Declaration {
            storage_class: None,
            ty: outer_array.into(),
            identifier: Identifier::new("three_dimensional")?,
            initializer: Some(
                InitializerList::Ordered(vec![
                    InitializerList::Ordered(vec![
                        InitializerList::Ordered(vec![
                            Value::signed_integer(1).into(),
                            Value::signed_integer(2).into(),
                            Value::signed_integer(3).into(),
                            Value::signed_integer(4).into(),
                        ])
                        .into(),
                        InitializerList::Ordered(vec![
                            Value::signed_integer(5).into(),
                            Value::signed_integer(6).into(),
                            Value::signed_integer(7).into(),
                            Value::signed_integer(8).into(),
                        ])
                        .into(),
                        InitializerList::Ordered(vec![
                            Value::signed_integer(9).into(),
                            Value::signed_integer(10).into(),
                            Value::signed_integer(11).into(),
                            Value::signed_integer(12).into(),
                        ])
                        .into(),
                    ])
                    .into(),
                    InitializerList::Ordered(vec![
                        InitializerList::Ordered(vec![
                            Value::signed_integer(13).into(),
                            Value::signed_integer(14).into(),
                            Value::signed_integer(15).into(),
                            Value::signed_integer(16).into(),
                        ])
                        .into(),
                        InitializerList::Ordered(vec![
                            Value::signed_integer(17).into(),
                            Value::signed_integer(18).into(),
                            Value::signed_integer(19).into(),
                            Value::signed_integer(20).into(),
                        ])
                        .into(),
                        InitializerList::Ordered(vec![
                            Value::signed_integer(21).into(),
                            Value::signed_integer(22).into(),
                            Value::signed_integer(23).into(),
                            Value::signed_integer(24).into(),
                        ])
                        .into(),
                    ])
                    .into(),
                ])
                .into(),
            ),
        });
        assert_eq!(
            declaration.to_string(),
            "int three_dimensional[2][3][4] = { { { 1, 2, 3, 4 }, { 5, 6, 7, 8 }, { 9, 10, 11, 12 } }, { { 13, 14, 15, 16 }, { 17, 18, 19, 20 }, { 21, 22, 23, 24 } } };"
        );

        Ok(())
    }

    #[test]
    fn array_of_strings() -> anyhow::Result<()> {
        let definition = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Array {
                element_type: Box::new(ConcreteType::Char),
                size: Some(26),
            }
            .into(),
            identifier: Identifier::new("blue")?,
            initializer: None,
        });
        assert_eq!(definition.to_string(), "char blue[26];");

        let fixed_char = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Array {
                element_type: Box::new(ConcreteType::Char),
                size: Some(26),
            }
            .into(),
            identifier: Identifier::new("yellow")?,
            initializer: Some(
                InitializerList::Ordered(
                    "yellow\0"
                        .chars()
                        .map(|value| Value::Char { value }.into())
                        .collect(),
                )
                .into(),
            ),
        });
        assert_eq!(
            fixed_char.to_string(),
            "char yellow[26] = { 'y', 'e', 'l', 'l', 'o', 'w', '\0' };"
        );

        let fixed_string = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Array {
                element_type: Box::new(ConcreteType::Char),
                size: Some(26),
            }
            .into(),
            identifier: Identifier::new("orange")?,
            initializer: Some(Value::String("orange".to_string()).into()),
        });
        assert_eq!(fixed_string.to_string(), r#"char orange[26] = "orange";"#);

        let flexible_char = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Array {
                element_type: Box::new(ConcreteType::Char),
                size: None,
            }
            .into(),
            identifier: Identifier::new("gray")?,
            initializer: Some(
                InitializerList::Ordered(
                    "gray\0"
                        .chars()
                        .map(|value| Value::Char { value }.into())
                        .collect(),
                )
                .into(),
            ),
        });
        assert_eq!(
            flexible_char.to_string(),
            "char gray[] = { 'g', 'r', 'a', 'y', '\0' };"
        );

        let flexible_string = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Array {
                element_type: Box::new(ConcreteType::Char),
                size: None,
            }
            .into(),
            identifier: Identifier::new("salmon")?,
            initializer: Some(Value::String("salmon".to_string()).into()),
        });
        assert_eq!(flexible_string.to_string(), r#"char salmon[] = "salmon";"#);

        Ok(())
    }

    #[test]
    fn array_of_structures() -> anyhow::Result<()> {
        let definition = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Array {
                element_type: Box::new(
                    Struct::Tag {
                        name: Identifier::new("point")?,
                    }
                    .into(),
                ),

                size: Some(3),
            }
            .into(),
            identifier: Identifier::new("point_array")?,
            initializer: None,
        });
        assert_eq!(definition.to_string(), "struct point point_array[3];");

        let declaration = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Array {
                element_type: Box::new(
                    Struct::Tag {
                        name: Identifier::new("point")?,
                    }
                    .into(),
                ),
                size: Some(3),
            }
            .into(),
            identifier: Identifier::new("point_array")?,
            initializer: Some(
                InitializerList::Ordered(vec![
                    InitializerList::Ordered(vec![Value::signed_integer(2).into(), Value::signed_integer(3).into()])
                        .into(),
                    InitializerList::Ordered(vec![Value::signed_integer(4).into(), Value::signed_integer(5).into()])
                        .into(),
                    InitializerList::Ordered(vec![Value::signed_integer(6).into(), Value::signed_integer(7).into()])
                        .into(),
                ])
                .into(),
            ),
        });
        assert_eq!(
            declaration.to_string(),
            "struct point point_array[3] = { { 2, 3 }, { 4, 5 }, { 6, 7 } };"
        );

        let member_access = Statement::Expression(
            Assignment {
                left: MemberAccess {
                    left: ArraySubscript {
                        array: Variable::new("point_array")?.into(),
                        index: Value::signed_integer(0).into(),
                    }
                    .into(),
                    member: Identifier::new("x")?,
                }
                .into(),
                right: Value::signed_integer(2).into(),
            }
            .into(),
        );
        assert_eq!(member_access.to_string(), "point_array[0].x = 2;");

        Ok(())
    }

    #[test]
    fn array_of_unions() -> anyhow::Result<()> {
        let definition = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Array {
                element_type: Box::new(
                    Union::Tag {
                        name: Identifier::new("numbers")?,
                    }
                    .into(),
                ),
                size: Some(3),
            }
            .into(),
            identifier: Identifier::new("number_array")?,
            initializer: None,
        });
        assert_eq!(definition.to_string(), "union numbers number_array[3];");

        let declaration = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Array {
                element_type: Box::new(
                    Union::Tag {
                        name: Identifier::new("numbers")?,
                    }
                    .into(),
                ),
                size: Some(3),
            }
            .into(),
            identifier: Identifier::new("number_array")?,
            initializer: Some(
                InitializerList::Ordered(vec![
                    InitializerList::Ordered(vec![Value::signed_integer(3).into()]).into(),
                    InitializerList::Ordered(vec![Value::signed_integer(4).into()]).into(),
                    InitializerList::Ordered(vec![Value::signed_integer(5).into()]).into(),
                ])
                .into(),
            ),
        });
        assert_eq!(
            declaration.to_string(),
            "union numbers number_array[3] = { { 3 }, { 4 }, { 5 } };"
        );

        let member_access = Statement::Expression(
            Assignment {
                left: MemberAccess {
                    left: ArraySubscript {
                        array: Variable::new("number_array")?.into(),
                        index: Value::signed_integer(0).into(),
                    }
                    .into(),
                    member: Identifier::new("i")?,
                }
                .into(),

                right: Value::signed_integer(2).into(),
            }
            .into(),
        );
        assert_eq!(member_access.to_string(), "number_array[0].i = 2;");

        Ok(())
    }
}
