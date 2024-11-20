use pretty::Pretty;

use crate::{
    macros::impl_froms,
    operator::{
        Assignment, BinaryOperator, Cast, CompoundAssignment, PostfixOperator, PrefixOperator,
        SizeOf,
    },
    pretty::impl_display_via_pretty,
    Identifier, Value,
};

#[derive(Clone)]
pub enum Expression {
    Assignment(Box<Assignment>),
    BinaryOperator(Box<BinaryOperator>),
    Cast(Box<Cast>),
    CompoundAssignment(Box<CompoundAssignment>),
    FunctionCall {
        name: Identifier,
        arguments: Vec<Box<Expression>>,
    },
    PostfixOperator(PostfixOperator),
    PrefixOperator(PrefixOperator),
    SizeOf(Box<SizeOf>),
    Custom(String),
    Value(Value),
    Variable(Identifier),
}

impl_froms!(Expression:
    box Assignment,
    box BinaryOperator,
    box Cast,
    box CompoundAssignment,
    PostfixOperator,
    PrefixOperator,
    box SizeOf,
    Value
);

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Expression
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        match self {
            Expression::Assignment(assignment) => assignment.pretty(allocator),
            Expression::BinaryOperator(operation) => operation.pretty(allocator),
            Expression::Cast(cast) => cast.pretty(allocator),
            Expression::CompoundAssignment(assignment) => assignment.pretty(allocator),
            Expression::FunctionCall { name, arguments } => allocator
                .text(name)
                .append(allocator.space())
                .append(allocator.text("("))
                .append(allocator.intersperse(
                    arguments.into_iter().map(|arg| arg.pretty(allocator)),
                    allocator.text(",").append(allocator.space()),
                ))
                .append(allocator.text(")")),
            Expression::PrefixOperator(operation) => operation.pretty(allocator),
            Expression::PostfixOperator(operation) => operation.pretty(allocator),
            Expression::SizeOf(sizeof) => sizeof.pretty(allocator),
            Expression::Custom(s) => allocator.text(s),
            Expression::Value(value) => allocator.text(value.to_string()),
            Expression::Variable(variable) => allocator.text(variable),
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
            arguments: vec![Box::new(Value::int(5).into())],
        })
        .to_string();

        assert_eq!(generated, "foo (5);");

        Ok(())
    }
}
