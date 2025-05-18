use pretty::Pretty;

use crate::Identifier;

use super::{impl_display_via_pretty, Statement};

#[derive(Clone, Debug)]
pub struct Label {
    pub identifier: Identifier,
    pub statement: Option<Statement>,
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

        if let Some(statement) = self.statement {
            // Don't add a newline before a label, as it will be added by the label itself to guarantee the correct indentation
            let builder = if !matches!(statement, Statement::Label(_)) {
                label.append(allocator.hardline())
            } else {
                label
            };

            builder.append(statement.pretty(allocator))
        } else {
            label
        }
    }
}

impl_display_via_pretty!(Label, 80);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{function, Block, IncompleteType, Value};

    #[test]
    fn generation() -> anyhow::Result<()> {
        let generated = function::Definition {
            is_static: false,
            name: Identifier::new("main")?,
            parameters: Vec::new(),
            return_ty: IncompleteType::Void,
            body: Block {
                statements: vec![Label {
                    identifier: Identifier::new("loop_start")?,
                    statement: Some(Statement::Expression(Value::int(42).into())),
                }
                .into()],
            },
        }
        .to_string();
        assert_eq!(
            generated,
            r#"void
main () {
loop_start:
  42;
}"#
        );

        Ok(())
    }

    #[test]
    fn inside_block() -> anyhow::Result<()> {
        let generated = function::Definition {
            is_static: false,
            name: Identifier::new("main")?,
            parameters: Vec::new(),
            return_ty: IncompleteType::Void,
            body: Block {
                statements: vec![Block {
                    statements: vec![Label {
                        identifier: Identifier::new("loop_start")?,
                        statement: Some(Statement::Expression(Value::int(42).into())),
                    }
                    .into()],
                }
                .into()],
            },
        }
        .to_string();
        assert_eq!(
            generated,
            r#"void
main () {
  {
loop_start:
    42;
  }
}"#
        );

        Ok(())
    }

    #[test]
    fn double_label() -> anyhow::Result<()> {
        let generated = function::Definition {
            is_static: false,
            name: Identifier::new("main")?,
            parameters: Vec::new(),
            return_ty: IncompleteType::Void,
            body: Block {
                statements: vec![Block {
                    statements: vec![
                        Label {
                            identifier: Identifier::new("loop_start")?,
                            statement: None,
                        }
                        .into(),
                        Label {
                            identifier: Identifier::new("loop_start2")?,
                            statement: None,
                        }
                        .into(),
                        Statement::Expression(Value::int(42).into()),
                    ],
                }
                .into()],
            },
        }
        .to_string();
        assert_eq!(
            generated,
            r#"void
main () {
  {
loop_start:
loop_start2:
    42;
  }
}"#
        );

        Ok(())
    }

    #[test]
    fn nested_labels() -> anyhow::Result<()> {
        let generated = function::Definition {
            is_static: false,
            name: Identifier::new("main")?,
            parameters: Vec::new(),
            return_ty: IncompleteType::Void,
            body: Block {
                statements: vec![Block {
                    statements: vec![
                        Label {
                            identifier: Identifier::new("loop_start")?,
                            statement: Some(
                                Label {
                                    identifier: Identifier::new("loop_start2")?,
                                    statement: None,
                                }
                                .into(),
                            ),
                        }
                        .into(),
                        Statement::Expression(Value::int(42).into()),
                    ],
                }
                .into()],
            },
        }
        .to_string();
        assert_eq!(
            generated,
            r#"void
main () {
  {
loop_start:
loop_start2:
    42;
  }
}"#
        );

        Ok(())
    }
}
