use pretty::Pretty;

use crate::{pretty::impl_display_via_pretty, Type};

/// Represents a C array type with its base type and size
#[derive(Clone)]
pub struct Array {
    /// The base type of the array elements
    pub element_type: Box<Type>,
    /// The size of the array (optional for flexible arrays)
    pub size: Option<usize>,
}

impl Array {
    pub fn base_type(&self) -> Type {
        match self.element_type.as_ref() {
            Type::Array(array) => array.base_type(),
            ty => ty.clone(),
        }
    }

    pub fn dimensions(&self) -> Vec<Option<usize>> {
        let mut dimensions = vec![self.size];

        if let Type::Array(array) = self.element_type.as_ref() {
            dimensions.extend(array.dimensions())
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
        let base_type = self.base_type().pretty(allocator);
        let dimensions = self.pretty_dimensions(allocator);

        base_type.append(dimensions)
    }
}

impl_display_via_pretty!(Array, 80);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{r#type::InitializerList, variable, Identifier, Statement, Value};

    #[test]
    fn fixed_width() -> anyhow::Result<()> {
        let definition = variable::Definition {
            storage_class: None,
            ty: Array {
                element_type: Box::new(Type::int()),
                size: Some(10),
            }
            .into(),
            identifiers: vec![Identifier::new("my_array")?].try_into()?,
        };
        assert_eq!(definition.to_string(), "int my_array[10];");

        let initialization = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Array {
                element_type: Box::new(Type::int()),
                size: Some(10),
            }
            .into(),
            variables: vec![(
                Identifier::new("my_array")?,
                Some(
                    InitializerList::Ordered(vec![
                        Value::int(0).into(),
                        Value::int(1).into(),
                        Value::int(2).into(),
                        Value::int(3).into(),
                        Value::int(4).into(),
                    ])
                    .into(),
                ),
            )]
            .try_into()?,
        });
        assert_eq!(
            initialization.to_string(),
            "int my_array[10] = { 0, 1, 2, 3, 4 };"
        );

        Ok(())
    }

    #[test]
    fn flex_width() -> anyhow::Result<()> {
        let definition = variable::Definition {
            storage_class: None,
            ty: Array {
                element_type: Box::new(Type::int()),
                size: None,
            }
            .into(),
            identifiers: vec![Identifier::new("flex")?].try_into()?,
        };
        assert_eq!(definition.to_string(), "int flex[];");

        let initialization = Statement::from(variable::Declaration {
            storage_class: None,
            ty: Array {
                element_type: Box::new(Type::int()),
                size: None,
            }
            .into(),
            variables: vec![(
                Identifier::new("flex")?,
                Some(
                    InitializerList::Ordered(vec![
                        Value::int(0).into(),
                        Value::int(1).into(),
                        Value::int(2).into(),
                        Value::int(3).into(),
                        Value::int(4).into(),
                    ])
                    .into(),
                ),
            )]
            .try_into()?,
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
            element_type: Box::new(Type::int()),
            size: Some(5),
        };

        let outer_array = Array {
            element_type: Box::new(inner_array.into()),
            size: Some(2),
        };

        let definition = Statement::from(variable::Definition {
            storage_class: None,
            ty: outer_array.clone().into(),
            identifiers: vec![Identifier::new("two_dimensions")?].try_into()?,
        });
        assert_eq!(definition.to_string(), "int two_dimensions[2][5];");

        let declaration = Statement::from(variable::Declaration {
            storage_class: None,
            ty: outer_array.into(),
            variables: vec![(
                Identifier::new("two_dimensions")?,
                Some(
                    InitializerList::Ordered(vec![
                        InitializerList::Ordered(vec![
                            Value::int(1).into(),
                            Value::int(2).into(),
                            Value::int(3).into(),
                            Value::int(4).into(),
                            Value::int(5).into(),
                        ])
                        .into(),
                        InitializerList::Ordered(vec![
                            Value::int(6).into(),
                            Value::int(7).into(),
                            Value::int(8).into(),
                            Value::int(9).into(),
                            Value::int(10).into(),
                        ])
                        .into(),
                    ])
                    .into(),
                ),
            )]
            .try_into()?,
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
            element_type: Box::new(Type::int()),
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

        let definition = Statement::from(variable::Definition {
            storage_class: None,
            ty: outer_array.clone().into(),
            identifiers: vec![Identifier::new("three_dimensional")?].try_into()?,
        });
        assert_eq!(definition.to_string(), "int three_dimensional[2][3][4];");

        let declaration = Statement::from(variable::Declaration {
            storage_class: None,
            ty: outer_array.into(),
            variables: vec![(
                Identifier::new("three_dimensional")?,
                Some(
                    InitializerList::Ordered(vec![
                        InitializerList::Ordered(vec![
                            InitializerList::Ordered(vec![
                                Value::int(1).into(),
                                Value::int(2).into(),
                                Value::int(3).into(),
                                Value::int(4).into(),
                            ])
                            .into(),
                            InitializerList::Ordered(vec![
                                Value::int(5).into(),
                                Value::int(6).into(),
                                Value::int(7).into(),
                                Value::int(8).into(),
                            ])
                            .into(),
                            InitializerList::Ordered(vec![
                                Value::int(9).into(),
                                Value::int(10).into(),
                                Value::int(11).into(),
                                Value::int(12).into(),
                            ])
                            .into(),
                        ])
                        .into(),
                        InitializerList::Ordered(vec![
                            InitializerList::Ordered(vec![
                                Value::int(13).into(),
                                Value::int(14).into(),
                                Value::int(15).into(),
                                Value::int(16).into(),
                            ])
                            .into(),
                            InitializerList::Ordered(vec![
                                Value::int(17).into(),
                                Value::int(18).into(),
                                Value::int(19).into(),
                                Value::int(20).into(),
                            ])
                            .into(),
                            InitializerList::Ordered(vec![
                                Value::int(21).into(),
                                Value::int(22).into(),
                                Value::int(23).into(),
                                Value::int(24).into(),
                            ])
                            .into(),
                        ])
                        .into(),
                    ])
                    .into(),
                ),
            )]
            .try_into()?,
        });
        assert_eq!(
            declaration.to_string(),
            "int three_dimensional[2][3][4] = { { { 1, 2, 3, 4 }, { 5, 6, 7, 8 }, { 9, 10, 11, 12 } }, { { 13, 14, 15, 16 }, { 17, 18, 19, 20 }, { 21, 22, 23, 24 } } };"
        );

        Ok(())
    }
}
