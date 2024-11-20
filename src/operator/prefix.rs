use crate::{pretty::impl_display_via_pretty, Identifier};
use pretty::Pretty;
use std::fmt;

/// Represents prefix operations like:
/// - increment (++x)
/// - decrement (--x)
/// - positive (+x)
/// - negative (-x)
/// - address-of (&x)
/// - indirection (*x)
#[derive(Clone)]
pub struct PrefixOperator {
    pub variable: Identifier,
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
            .append(self.variable)
    }
}

impl_display_via_pretty!(PrefixOperator, 80);

#[derive(Clone, Copy)]
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
    use crate::CStatement;

    use super::*;

    #[test]
    fn increment_decrement() -> anyhow::Result<()> {
        let increment = CStatement::Expression(
            PrefixOperator {
                variable: "x".to_string(),
                operator: PrefixOperatorKind::Increment,
            }
            .into(),
        )
        .to_string();

        assert_eq!(increment, "++x;");

        let decrement = CStatement::Expression(
            PrefixOperator {
                variable: "y".to_string(),
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
        let positive = CStatement::Expression(
            PrefixOperator {
                variable: "x".to_string(),
                operator: PrefixOperatorKind::Positive,
            }
            .into(),
        )
        .to_string();

        assert_eq!(positive, "+x;");

        let negative = CStatement::Expression(
            PrefixOperator {
                variable: "y".to_string(),
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
        let address = CStatement::Expression(
            PrefixOperator {
                variable: "x".to_string(),
                operator: PrefixOperatorKind::Address,
            }
            .into(),
        )
        .to_string();

        assert_eq!(address, "&x;");

        let indirection = CStatement::Expression(
            PrefixOperator {
                variable: "ptr".to_string(),
                operator: PrefixOperatorKind::Indirection,
            }
            .into(),
        )
        .to_string();

        assert_eq!(indirection, "*ptr;");

        Ok(())
    }
}
