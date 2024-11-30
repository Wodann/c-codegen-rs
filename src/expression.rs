use pretty::Pretty;

use crate::{
    function::FunctionCall,
    macros::impl_froms,
    operator::{
        ArraySubscript, Assignment, BinaryOperator, Cast, CommaOperator, CompoundAssignment,
        PostfixOperator, PrefixOperator, SizeOf,
    },
    pretty::impl_display_via_pretty,
    r#type::{
        member::{IndirectMemberAccess, MemberAccess},
        InitializerList,
    },
    Identifier, Type, Value,
};

#[derive(Clone)]
pub enum Expression {
    AlignOf(Type),
    ArraySubscript(Box<ArraySubscript>),
    Assignment(Box<Assignment>),
    BinaryOperator(Box<BinaryOperator>),
    Cast(Box<Cast>),
    CommaOperator(Box<CommaOperator>),
    CompoundAssignment(Box<CompoundAssignment>),
    FunctionCall(FunctionCall),
    IndirectMemberAccess(Box<IndirectMemberAccess>),
    InitializerList(InitializerList),
    MemberAccess(MemberAccess),
    Parentheses(Box<Expression>),
    PostfixOperator(Box<PostfixOperator>),
    PrefixOperator(Box<PrefixOperator>),
    SizeOf(Box<SizeOf>),
    Value(Value),
    Variable(Identifier),
}

impl_froms!(Expression:
    box ArraySubscript,
    box Assignment,
    box BinaryOperator,
    box Cast,
    box CommaOperator,
    box CompoundAssignment,
    FunctionCall,
    box IndirectMemberAccess,
    InitializerList,
    MemberAccess,
    box PostfixOperator,
    box PrefixOperator,
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
            Expression::AlignOf(ty) => allocator
                .text("_Alignof")
                .append(allocator.text(ty.to_string()).parens()),
            Expression::ArraySubscript(array_subscript) => array_subscript.pretty(allocator),
            Expression::Assignment(assignment) => assignment.pretty(allocator),
            Expression::BinaryOperator(operation) => operation.pretty(allocator),
            Expression::Cast(cast) => cast.pretty(allocator),
            Expression::CommaOperator(comma) => comma.pretty(allocator),
            Expression::CompoundAssignment(assignment) => assignment.pretty(allocator),
            Expression::FunctionCall(function_call) => function_call.pretty(allocator),
            Expression::IndirectMemberAccess(member_access) => member_access.pretty(allocator),
            Expression::InitializerList(initializer_list) => initializer_list.pretty(allocator),
            Expression::MemberAccess(member_access) => member_access.pretty(allocator),
            Expression::Parentheses(expr) => expr.pretty(allocator).parens(),
            Expression::PrefixOperator(operation) => operation.pretty(allocator),
            Expression::PostfixOperator(operation) => operation.pretty(allocator),
            Expression::SizeOf(sizeof) => sizeof.pretty(allocator),
            Expression::Value(value) => allocator.text(value.to_string()),
            Expression::Variable(variable) => allocator.text(variable),
        }
    }
}

impl_display_via_pretty!(Expression, 80);

#[cfg(test)]
mod tests {
    use crate::{
        operator::{BinaryOperator, BinaryOperatorKind},
        Statement,
    };

    use super::*;

    #[test]
    fn alignof() -> anyhow::Result<()> {
        let generated = Statement::Expression(Expression::AlignOf(Type::int())).to_string();
        assert_eq!(generated, "_Alignof(int);");

        Ok(())
    }

    // Source: https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Calling-Functions
    #[test]
    fn function_call() -> anyhow::Result<()> {
        let generated = Statement::Expression(
            FunctionCall {
                name: Identifier::new("foo")?,
                arguments: vec![Value::int(5).into()],
            }
            .into(),
        )
        .to_string();

        assert_eq!(generated, "foo(5);");

        Ok(())
    }

    // Source: Chapter 3.1 Expressions example: ( 2 * ( ( 3 + 10 ) - ( 2 * 6 ) ) )
    #[test]
    fn parentheses_groups() -> anyhow::Result<()> {
        // Build from inside out:
        // First inner group (3 + 10)
        let inner_sum = Expression::Parentheses(Box::new(
            BinaryOperator {
                left: Value::int(3).into(),
                operator: BinaryOperatorKind::Add,
                right: Value::int(10).into(),
            }
            .into(),
        ));

        // Second inner group (2 * 6)
        let inner_product = Expression::Parentheses(Box::new(
            BinaryOperator {
                left: Value::int(2).into(),
                operator: BinaryOperatorKind::Mul,
                right: Value::int(6).into(),
            }
            .into(),
        ));

        // Middle group ((3 + 10) - (2 * 6))
        let middle_diff = Expression::Parentheses(Box::new(
            BinaryOperator {
                left: inner_sum,
                operator: BinaryOperatorKind::Sub,
                right: inner_product,
            }
            .into(),
        ));

        // Final expression: (2 * ((3 + 10) - (2 * 6)))
        let expr = Expression::Parentheses(Box::new(
            BinaryOperator {
                left: Value::int(2).into(),
                operator: BinaryOperatorKind::Mul,
                right: middle_diff,
            }
            .into(),
        ));

        let generated = Statement::Expression(expr).to_string();
        assert_eq!(generated, "(2 * ((3 + 10) - (2 * 6)));");

        Ok(())
    }
}
