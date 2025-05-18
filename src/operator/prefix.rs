use crate::{pretty::impl_display_via_pretty, Expression};
use pretty::Pretty;
use std::fmt;

/// Represents prefix operations like:
/// - increment (++x)
/// - decrement (--x)
/// - positive (+x)
/// - negative (-x)
/// - address-of (&x)
/// - indirection (*x)
#[derive(Clone, Debug)]
pub struct PrefixOperator {
    pub operand: Expression,
    pub operator: PrefixOperatorKind,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for PrefixOperator
where
    AnnotationT: Clone + 'a,
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        allocator
            .text(self.operator.to_string())
            .append(self.operand.pretty(allocator))
    }
}

impl_display_via_pretty!(PrefixOperator, 80);

#[derive(Clone, Copy, Debug)]
pub enum PrefixOperatorKind {
    Increment,   // ++x
    Decrement,   // --x
    Positive,    // +x
    Negative,    // -x
    Address,     // &x
    Indirection, // *x
}

impl fmt::Display for PrefixOperatorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            PrefixOperatorKind::Increment => "++",
            PrefixOperatorKind::Decrement => "--",
            PrefixOperatorKind::Positive => "+",
            PrefixOperatorKind::Negative => "-",
            PrefixOperatorKind::Address => "&",
            PrefixOperatorKind::Indirection => "*",
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Statement, Variable};

    use super::*;

    #[test]
    fn increment_decrement() -> anyhow::Result<()> {
        let increment = Statement::Expression(
            PrefixOperator {
                operand: Variable::new("x")?.into(),
                operator: PrefixOperatorKind::Increment,
            }
            .into(),
        )
        .to_string();

        assert_eq!(increment, "++x;");

        let decrement = Statement::Expression(
            PrefixOperator {
                operand: Variable::new("y")?.into(),
                operator: PrefixOperatorKind::Decrement,
            }
            .into(),
        )
        .to_string();

        assert_eq!(decrement, "--y;");

        Ok(())
    }

    #[test]
    fn positive_negative() -> anyhow::Result<()> {
        let positive = Statement::Expression(
            PrefixOperator {
                operand: Variable::new("x")?.into(),
                operator: PrefixOperatorKind::Positive,
            }
            .into(),
        )
        .to_string();

        assert_eq!(positive, "+x;");

        let negative = Statement::Expression(
            PrefixOperator {
                operand: Variable::new("y")?.into(),
                operator: PrefixOperatorKind::Negative,
            }
            .into(),
        )
        .to_string();

        assert_eq!(negative, "-y;");

        Ok(())
    }

    #[test]
    fn pointer() -> anyhow::Result<()> {
        let address = Statement::Expression(
            PrefixOperator {
                operand: Variable::new("x")?.into(),
                operator: PrefixOperatorKind::Address,
            }
            .into(),
        )
        .to_string();

        assert_eq!(address, "&x;");

        let indirection = Statement::Expression(
            PrefixOperator {
                operand: Variable::new("ptr")?.into(),
                operator: PrefixOperatorKind::Indirection,
            }
            .into(),
        )
        .to_string();

        assert_eq!(indirection, "*ptr;");

        Ok(())
    }
}
