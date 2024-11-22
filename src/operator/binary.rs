use core::fmt;
use pretty::Pretty;

use crate::{pretty::impl_display_via_pretty, Expression};

#[derive(Clone)]
pub struct BinaryOperator {
    pub left: Expression,
    pub operator: BinaryOperatorKind,
    pub right: Expression,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for BinaryOperator
where
    AnnotationT: Clone + 'a,
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        self.left
            .pretty(allocator)
            .append(allocator.space())
            .append(allocator.text(self.operator.to_string()))
            .append(allocator.space())
            .append(self.right.pretty(allocator))
    }
}

impl_display_via_pretty!(BinaryOperator, 80);

#[derive(Clone, Copy)]
pub enum BinaryOperatorKind {
    Add,    // +
    Sub,    // -
    Mul,    // *
    Div,    // /
    Mod,    // %
    Eq,     // ==
    Ne,     // !=
    Lt,     // <
    Le,     // <=
    Gt,     // >
    Ge,     // >=
    And,    // &&
    Or,     // ||
    LShift, // <<
    RShift, // >>
    BitAnd, // &
    BitOr,  // |
    BitXor, // ^
}

impl fmt::Display for BinaryOperatorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            BinaryOperatorKind::Add => "+",
            BinaryOperatorKind::Sub => "-",
            BinaryOperatorKind::Mul => "*",
            BinaryOperatorKind::Div => "/",
            BinaryOperatorKind::Mod => "%",
            BinaryOperatorKind::Eq => "==",
            BinaryOperatorKind::Ne => "!=",
            BinaryOperatorKind::Lt => "<",
            BinaryOperatorKind::Le => "<=",
            BinaryOperatorKind::Gt => ">",
            BinaryOperatorKind::Ge => ">=",
            BinaryOperatorKind::And => "&&",
            BinaryOperatorKind::Or => "||",
            BinaryOperatorKind::LShift => "<<",
            BinaryOperatorKind::RShift => ">>",
            BinaryOperatorKind::BitAnd => "&",
            BinaryOperatorKind::BitOr => "|",
            BinaryOperatorKind::BitXor => "^",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Statement, Identifier};

    #[test]
    fn arithmetic() -> anyhow::Result<()> {
        let addition = Statement::Expression(
            BinaryOperator {
                left: Expression::Variable(Identifier::new("x")?),
                operator: BinaryOperatorKind::Add,
                right: Expression::Variable(Identifier::new("y")?),
            }
            .into(),
        )
        .to_string();
        assert_eq!(addition, "x + y;");

        let subtraction = Statement::Expression(
            BinaryOperator {
                left: Expression::Variable(Identifier::new("x")?),
                operator: BinaryOperatorKind::Sub,
                right: Expression::Variable(Identifier::new("y")?),
            }
            .into(),
        )
        .to_string();
        assert_eq!(subtraction, "x - y;");

        let multiplication = Statement::Expression(
            BinaryOperator {
                left: Expression::Variable(Identifier::new("x")?),
                operator: BinaryOperatorKind::Mul,
                right: Expression::Variable(Identifier::new("y")?),
            }
            .into(),
        )
        .to_string();
        assert_eq!(multiplication, "x * y;");

        let division = Statement::Expression(
            BinaryOperator {
                left: Expression::Variable(Identifier::new("x")?),
                operator: BinaryOperatorKind::Div,
                right: Expression::Variable(Identifier::new("y")?),
            }
            .into(),
        )
        .to_string();
        assert_eq!(division, "x / y;");

        let modulo = Statement::Expression(
            BinaryOperator {
                left: Expression::Variable(Identifier::new("x")?),
                operator: BinaryOperatorKind::Mod,
                right: Expression::Variable(Identifier::new("y")?),
            }
            .into(),
        )
        .to_string();
        assert_eq!(modulo, "x % y;");

        Ok(())
    }

    #[test]
    fn comparison() -> anyhow::Result<()> {
        let equal_to = Statement::Expression(
            BinaryOperator {
                left: Expression::Variable(Identifier::new("x")?),
                operator: BinaryOperatorKind::Eq,
                right: Expression::Variable(Identifier::new("y")?),
            }
            .into(),
        )
        .to_string();
        assert_eq!(equal_to, "x == y;");

        let not_equal_to = Statement::Expression(
            BinaryOperator {
                left: Expression::Variable(Identifier::new("x")?),
                operator: BinaryOperatorKind::Ne,
                right: Expression::Variable(Identifier::new("y")?),
            }
            .into(),
        )
        .to_string();
        assert_eq!(not_equal_to, "x != y;");

        let less_than = Statement::Expression(
            BinaryOperator {
                left: Expression::Variable(Identifier::new("x")?),
                operator: BinaryOperatorKind::Lt,
                right: Expression::Variable(Identifier::new("y")?),
            }
            .into(),
        )
        .to_string();
        assert_eq!(less_than, "x < y;");

        let less_than_or_equal = Statement::Expression(
            BinaryOperator {
                left: Expression::Variable(Identifier::new("x")?),
                operator: BinaryOperatorKind::Le,
                right: Expression::Variable(Identifier::new("y")?),
            }
            .into(),
        )
        .to_string();
        assert_eq!(less_than_or_equal, "x <= y;");

        let greater_than = Statement::Expression(
            BinaryOperator {
                left: Expression::Variable(Identifier::new("x")?),
                operator: BinaryOperatorKind::Gt,
                right: Expression::Variable(Identifier::new("y")?),
            }
            .into(),
        )
        .to_string();
        assert_eq!(greater_than, "x > y;");

        let greater_than_or_equal = Statement::Expression(
            BinaryOperator {
                left: Expression::Variable(Identifier::new("x")?),
                operator: BinaryOperatorKind::Ge,
                right: Expression::Variable(Identifier::new("y")?),
            }
            .into(),
        )
        .to_string();
        assert_eq!(greater_than_or_equal, "x >= y;");

        Ok(())
    }

    #[test]
    fn logical() -> anyhow::Result<()> {
        let logical_and = Statement::Expression(
            BinaryOperator {
                left: Expression::Variable(Identifier::new("x")?),
                operator: BinaryOperatorKind::And,
                right: Expression::Variable(Identifier::new("y")?),
            }
            .into(),
        )
        .to_string();
        assert_eq!(logical_and, "x && y;");

        let logical_or = Statement::Expression(
            BinaryOperator {
                left: Expression::Variable(Identifier::new("x")?),
                operator: BinaryOperatorKind::Or,
                right: Expression::Variable(Identifier::new("y")?),
            }
            .into(),
        )
        .to_string();
        assert_eq!(logical_or, "x || y;");

        Ok(())
    }

    #[test]
    fn bit_shifting() -> anyhow::Result<()> {
        let left_shift = Statement::Expression(
            BinaryOperator {
                left: Expression::Variable(Identifier::new("x")?),
                operator: BinaryOperatorKind::LShift,
                right: Expression::Variable(Identifier::new("y")?),
            }
            .into(),
        )
        .to_string();
        assert_eq!(left_shift, "x << y;");

        let right_shift = Statement::Expression(
            BinaryOperator {
                left: Expression::Variable(Identifier::new("x")?),
                operator: BinaryOperatorKind::RShift,
                right: Expression::Variable(Identifier::new("y")?),
            }
            .into(),
        )
        .to_string();
        assert_eq!(right_shift, "x >> y;");

        Ok(())
    }

    #[test]
    fn bitwise_logical() -> anyhow::Result<()> {
        let bitwise_and = Statement::Expression(
            BinaryOperator {
                left: Expression::Variable(Identifier::new("x")?),
                operator: BinaryOperatorKind::BitAnd,
                right: Expression::Variable(Identifier::new("y")?),
            }
            .into(),
        )
        .to_string();
        assert_eq!(bitwise_and, "x & y;");

        let bitwise_or = Statement::Expression(
            BinaryOperator {
                left: Expression::Variable(Identifier::new("x")?),
                operator: BinaryOperatorKind::BitOr,
                right: Expression::Variable(Identifier::new("y")?),
            }
            .into(),
        )
        .to_string();
        assert_eq!(bitwise_or, "x | y;");

        let bitwise_xor = Statement::Expression(
            BinaryOperator {
                left: Expression::Variable(Identifier::new("x")?),
                operator: BinaryOperatorKind::BitXor,
                right: Expression::Variable(Identifier::new("y")?),
            }
            .into(),
        )
        .to_string();
        assert_eq!(bitwise_xor, "x ^ y;");

        Ok(())
    }
}
