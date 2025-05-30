use pretty::Pretty;

use crate::{r#type::Scalar, Expression};

#[derive(Clone, Debug)]
pub struct Cast {
    pub new_ty: Scalar,
    pub expression: Expression,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Cast
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        allocator
            .text("(")
            .append(allocator.text(self.new_ty.to_string()))
            .append(allocator.text(")"))
            .append(allocator.space())
            .append(self.expression.pretty(allocator))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Statement, Variable};

    use super::*;

    #[test]
    fn cast() -> anyhow::Result<()> {
        let cast = Statement::Expression(
            Cast {
                new_ty: Scalar::int(),
                expression: Variable::new("x")?.into(),
            }
            .into(),
        )
        .to_string();

        assert_eq!(cast, "(int) x;");

        // Cast (x / z) to float
        // TODO: Add a test for this once we support brackets

        Ok(())
    }
}
