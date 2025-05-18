use crate::{pretty::impl_display_via_pretty, Expression, ConcreteType};
use pretty::Pretty;

#[derive(Clone)]
pub enum SizeOf {
    Expression(Expression),
    Type(ConcreteType),
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for SizeOf
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let content = match self {
            SizeOf::Expression(expr) => expr.pretty(allocator),
            SizeOf::Type(typ) => allocator
                .text("(")
                .append(allocator.text(typ.to_string()))
                .append(allocator.text(")")),
        };

        allocator
            .text("sizeof")
            .append(allocator.space())
            .append(content)
    }
}

impl_display_via_pretty!(SizeOf, 80);

#[cfg(test)]
mod test {
    use crate::{Statement, Variable};

    use super::*;

    #[test]
    fn sizeof_operator() -> anyhow::Result<()> {
        // Test sizeof with a type
        let sizeof_type = Statement::Expression(SizeOf::Type(ConcreteType::int()).into()).to_string();

        assert_eq!(sizeof_type, "sizeof (int);");

        // Test sizeof with an expression
        let sizeof_expr = Statement::Expression(
            SizeOf::Expression(Expression::Variable(Variable::new("x")?)).into(),
        )
        .to_string();

        assert_eq!(sizeof_expr, "sizeof x;");

        Ok(())
    }
}
