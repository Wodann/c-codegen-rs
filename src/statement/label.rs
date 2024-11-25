use pretty::Pretty;

use crate::Identifier;

use super::{impl_display_via_pretty, Statement};

#[derive(Clone)]
pub struct Label {
    identifier: Identifier,
    statement: Statement,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Label
where
    AnnotationT: Clone + 'a,
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let label = allocator.nesting(move |level| {
            let negative_level = -isize::try_from(level).expect("nesting level is too large");

            allocator
                .hardline()
                .append(allocator.text(self.identifier.clone()))
                .append(allocator.text(":"))
                .nest(negative_level)
                .into_doc()
        });

        // Don't add a newline before a label, as it will be added by the label itself to guarantee the correct indentation
        let builder = if !matches!(self.statement, Statement::Label(_)) {
            label.append(allocator.hardline())
        } else {
            label
        };

        builder.append(self.statement.clone().pretty(allocator))
    }
}

impl_display_via_pretty!(Label, 80);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{function, Block, Type, Value};

    #[test]
    fn generation() -> anyhow::Result<()> {
        let indentation = function::Definition {
            is_static: false,
            name: Identifier::new("main")?,
            parameters: Vec::new(),
            return_ty: Type::Void,
            body: Block {
                statements: vec![Statement::from(Label {
                    identifier: Identifier::new("loop_start")?,
                    statement: Statement::Expression(Value::int(42).into()),
                })],
            },
        }
        .to_string();
        assert_eq!(
            indentation,
            r#"void
main () {
loop_start:
  42;
}"#
        );

        Ok(())
    }
}
