use pretty::Pretty;

use crate::{pretty::impl_display_via_pretty, Identifier};

#[derive(Clone)]
pub enum Expression {
    // https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Calling-Functions
    FunctionCall {
        name: Identifier,
        arguments: Vec<Box<Expression>>,
    },
    Custom(String),
}

// Implementing Pretty for Expression
impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Expression
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        match self {
            Expression::FunctionCall { name, arguments } => allocator
                .text(name)
                .append(allocator.space())
                .append(allocator.text("("))
                .append(allocator.intersperse(
                    arguments.into_iter().map(|arg| arg.pretty(allocator)),
                    allocator.text(",").append(allocator.space()),
                ))
                .append(allocator.text(")")),
            Expression::Custom(s) => allocator.text(s),
        }
    }
}

impl_display_via_pretty!(Expression, 80);

#[cfg(test)]
mod tests {
    use crate::CStatement;

    use super::*;

    // Source: https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Calling-Functions
    #[test]
    fn function_call() -> anyhow::Result<()> {
        let generated = CStatement::Expression(Expression::FunctionCall {
            name: "foo".to_string(),
            arguments: vec![Box::new(Expression::Custom("5".to_string()))],
        })
        .to_string();

        assert_eq!(generated, "foo (5);");

        Ok(())
    }
}
