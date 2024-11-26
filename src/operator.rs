mod array_subscript;
mod assignment;
mod binary;
mod cast;
mod comma;
mod compound_assignment;
mod postfix;
mod prefix;
mod sizeof;

pub use self::{
    array_subscript::ArraySubscript,
    assignment::Assignment,
    binary::{BinaryOperator, BinaryOperatorKind},
    cast::Cast,
    comma::CommaOperator,
    compound_assignment::{CompoundAssignment, CompoundAssignmentOperator},
    postfix::{PostfixOperator, PostfixOperatorKind},
    prefix::{PrefixOperator, PrefixOperatorKind},
    sizeof::SizeOf,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Expression, Identifier, Statement, Value};

    #[test]
    fn assignment() -> anyhow::Result<()> {
        let assignment = Statement::Expression(
            Assignment {
                left: Expression::Variable(Identifier::new("x")?),
                right: Value::int(42).into(),
            }
            .into(),
        );
        assert_eq!(assignment.to_string(), "x = 42;");

        let compound_assignment = Statement::Expression(
            CompoundAssignment {
                left: Expression::Variable(Identifier::new("x")?),
                operator: CompoundAssignmentOperator::Add,
                right: Expression::Variable(Identifier::new("y")?),
            }
            .into(),
        );
        assert_eq!(compound_assignment.to_string(), "x += y;");

        Ok(())
    }

    #[test]
    fn increment_decrement() -> anyhow::Result<()> {
        let prefix_inc = Statement::Expression(
            PrefixOperator {
                operand: Expression::Variable(Identifier::new("x")?),
                operator: PrefixOperatorKind::Increment,
            }
            .into(),
        );
        assert_eq!(prefix_inc.to_string(), "++x;");

        let postfix_dec = Statement::Expression(
            PostfixOperator {
                operand: Expression::Variable(Identifier::new("y")?),
                operator: PostfixOperatorKind::Decrement,
            }
            .into(),
        );
        assert_eq!(postfix_dec.to_string(), "y--;");

        Ok(())
    }

    #[test]
    fn binary_operations() -> anyhow::Result<()> {
        let add = Statement::Expression(
            BinaryOperator {
                left: Expression::Variable(Identifier::new("x")?),
                operator: BinaryOperatorKind::Add,
                right: Expression::Variable(Identifier::new("y")?),
            }
            .into(),
        );
        assert_eq!(add.to_string(), "x + y;");

        let mul = Statement::Expression(
            BinaryOperator {
                left: Expression::Variable(Identifier::new("a")?),
                operator: BinaryOperatorKind::Mul,
                right: Expression::Variable(Identifier::new("b")?),
            }
            .into(),
        );
        assert_eq!(mul.to_string(), "a * b;");

        Ok(())
    }

    #[test]
    fn prefix_unary() -> anyhow::Result<()> {
        let pos = Statement::Expression(
            PrefixOperator {
                operand: Expression::Variable(Identifier::new("x")?),
                operator: PrefixOperatorKind::Positive,
            }
            .into(),
        );
        assert_eq!(pos.to_string(), "+x;");

        let neg = Statement::Expression(
            PrefixOperator {
                operand: Expression::Variable(Identifier::new("y")?),
                operator: PrefixOperatorKind::Negative,
            }
            .into(),
        );
        assert_eq!(neg.to_string(), "-y;");

        Ok(())
    }
}
