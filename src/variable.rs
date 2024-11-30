use pretty::Pretty;

use crate::{
    non_empty_vec::NonEmptyVec, pretty::impl_display_via_pretty, Expression, Identifier,
    StorageClass, Type,
};

#[derive(Clone)]
pub struct Definition {
    pub storage_class: Option<StorageClass>,
    pub ty: Type,
    pub identifiers: NonEmptyVec<Identifier>,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Definition
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let base_type = self.ty.base_type();
        let dimensions = self.ty.pretty_dimensions(allocator);

        pretty_variable_type(self.storage_class, base_type, allocator)
            .append(allocator.space())
            .append(
                allocator.intersperse(
                    self.identifiers
                        .into_iter()
                        .map(|identifier| allocator.text(identifier).append(dimensions.clone())),
                    allocator.text(",").append(allocator.space()),
                ),
            )
            .append(allocator.text(";"))
    }
}

impl_display_via_pretty!(Definition, 80);

/// Variable declaration
///
/// Unsupported C language features:
///
/// ```c
/// int test = 1, test2[2] = {1, 2};
/// ```
#[derive(Clone)]
pub struct Declaration {
    pub storage_class: Option<StorageClass>,
    pub ty: Type,
    pub variables: NonEmptyVec<(Identifier, Option<Expression>)>,
}

// Implement Pretty for Declaration
impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Declaration
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let base_type = self.ty.base_type();
        let dimensions = self.ty.pretty_dimensions(allocator);

        pretty_variable_type(self.storage_class, base_type, allocator)
            .append(allocator.space())
            .append(allocator.intersperse(
                self.variables.into_iter().map(|(identifier, initializer)| {
                    let mut builder = allocator.text(identifier).append(dimensions.clone());

                    if let Some(initializer) = initializer {
                        builder = builder
                            .append(allocator.space())
                            .append(allocator.text("="))
                            .append(allocator.space())
                            .append(initializer.pretty(allocator));
                    }

                    builder
                }),
                allocator.text(",").append(allocator.space()),
            ))
    }
}

impl_display_via_pretty!(Declaration, 80);

fn pretty_variable_type<'a, AllocatorT, AnnotationT>(
    storage_class: Option<StorageClass>,
    ty: Type,
    allocator: &'a AllocatorT,
) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT>
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    let builder = if let Some(storage_class) = storage_class {
        allocator
            .text(storage_class.to_string())
            .append(allocator.space())
    } else {
        allocator.nil()
    };

    builder.append(allocator.text(ty.to_string()))
}

#[cfg(test)]
mod tests {
    use crate::{Statement, Value};

    use super::*;

    #[test]
    fn initializers() -> anyhow::Result<()> {
        let multiple = Statement::from(Declaration {
            storage_class: None,
            ty: Type::int(),
            variables: vec![
                (Identifier::new("x")?, None),
                (Identifier::new("y")?, Some(Value::int(5).into())),
            ]
            .try_into()?,
        })
        .to_string();
        assert_eq!(multiple, "int x, y = 5;");

        Ok(())
    }
}
