use pretty::Pretty;

use crate::Identifier;

use super::impl_display_via_pretty;

#[derive(Clone)]
pub struct Goto {
    pub label: Identifier,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Goto
where
    AnnotationT: Clone + 'a,
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        allocator
            .text("goto")
            .append(allocator.space())
            .append(allocator.text(self.label))
            .append(allocator.text(";"))
    }
}

impl_display_via_pretty!(Goto, 80);

#[cfg(test)]
mod tests {
    use crate::statement::Label;

    use super::*;

    #[test]
    fn simple_goto() -> anyhow::Result<()> {
        let generated = Goto {
            label: Identifier::new("end_of_program")?,
        }
        .to_string();

        assert_eq!(generated, "goto end_of_program;");
        Ok(())
    }

    #[test]
    fn goto_with_label() -> anyhow::Result<()> {
        // This test demonstrates a common goto pattern with a label
        use crate::Block;

        let goto = Goto {
            label: Identifier::new("end_of_program")?,
        };

        let generated = Block {
            statements: vec![
                goto.clone().into(),
                Label {
                    identifier: Identifier::new("end_of_program")?,
                    statement: None,
                }
                .into(),
            ],
        }
        .to_string();

        assert_eq!(
            generated,
            "{
  goto end_of_program;
end_of_program:
}"
        );
        Ok(())
    }
}
