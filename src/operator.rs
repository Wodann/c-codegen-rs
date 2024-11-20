mod assignment;
mod binary;
mod cast;
mod compound_assignment;
mod postfix;
mod prefix;
mod sizeof;

pub use self::{
    assignment::Assignment,
    binary::{BinaryOperator, BinaryOperatorKind},
    cast::Cast,
    compound_assignment::{CompoundAssignment, CompoundAssignmentOperator},
    postfix::{PostfixOperator, PostfixOperatorKind},
    prefix::{PrefixOperator, PrefixOperatorKind},
    sizeof::SizeOf,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CStatement, Expression};

    #[test]
    fn assignment() {
        let assignment = CStatement::Expression(
            Assignment {
                variable_name: "x".to_string(),
                expression: Expression::Custom("42".to_string()),
            }
            .into(),
        );
        assert_eq!(assignment.to_string(), "x = 42;");

        let compound_assignment = CStatement::Expression(
            CompoundAssignment {
                variable: "x".to_string(),
                operator: CompoundAssignmentOperator::Add,
                expression: Expression::Custom("y".to_string()),
            }
            .into(),
        );
        assert_eq!(compound_assignment.to_string(), "x += y;");
    }

    #[test]
    fn increment_decrement() {
        let prefix_inc = CStatement::Expression(
            PrefixOperator {
                variable: "x".to_string(),
                operator: PrefixOperatorKind::Increment,
            }
            .into(),
        );
        assert_eq!(prefix_inc.to_string(), "++x;");

        let postfix_dec = CStatement::Expression(Expression::PostfixOperator(PostfixOperator {
            variable: "y".to_string(),
            operator: PostfixOperatorKind::Decrement,
        }));
        assert_eq!(postfix_dec.to_string(), "y--;");
    }

    #[test]
    fn binary_operations() {
        let add = CStatement::Expression(
            BinaryOperator {
                left: Expression::Custom("x".to_string()),
                operator: BinaryOperatorKind::Add,
                right: Expression::Custom("y".to_string()),
            }
            .into(),
        );
        assert_eq!(add.to_string(), "x + y;");

        let mul = CStatement::Expression(
            BinaryOperator {
                left: Expression::Custom("a".to_string()),
                operator: BinaryOperatorKind::Mul,
                right: Expression::Custom("b".to_string()),
            }
            .into(),
        );
        assert_eq!(mul.to_string(), "a * b;");
    }

    #[test]
    fn prefix_unary() {
        let pos = CStatement::Expression(Expression::PrefixOperator(PrefixOperator {
            variable: "x".to_string(),
            operator: PrefixOperatorKind::Positive,
        }));
        assert_eq!(pos.to_string(), "+x;");

        let neg = CStatement::Expression(Expression::PrefixOperator(PrefixOperator {
            variable: "y".to_string(),
            operator: PrefixOperatorKind::Negative,
        }));
        assert_eq!(neg.to_string(), "-y;");
    }
}
