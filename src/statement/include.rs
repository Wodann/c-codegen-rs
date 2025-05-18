use pretty::Pretty;

use crate::pretty::impl_display_via_pretty;

#[derive(Clone, Debug)]
pub enum Style {
    AngleBrackets,
    Quotes,
}

#[derive(Clone, Debug)]
pub struct Include {
    pub path: String,
    pub style: Style,
}

impl Include {
    /// Creates a new instance using angle brackets.
    pub fn with_angle_brackets<S: Into<String>>(path: S) -> Self {
        Self {
            path: path.into(),
            style: Style::AngleBrackets,
        }
    }

    /// Creates a new instance using quotes.
    pub fn with_quotes<S: Into<String>>(path: S) -> Self {
        Self {
            path: path.into(),
            style: Style::Quotes,
        }
    }
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Include
where
    AnnotationT: Clone + 'a,
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let (pre, post) = match self.style {
            Style::AngleBrackets => ("<", ">"),
            Style::Quotes => ("\"", "\""),
        };

        allocator
            .text("#include")
            .append(allocator.space())
            .append(allocator.text(format!("{pre}{}{post}", self.path)))
    }
}

impl_display_via_pretty!(Include, 80);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_angle_brackets() {
        let generated = Include::with_angle_brackets("stdio.h").to_string();

        assert_eq!(generated, "#include <stdio.h>");
    }

    #[test]
    fn with_quotes() {
        let generated = Include::with_quotes("foo.h").to_string();

        assert_eq!(generated, "#include \"foo.h\"");
    }
}
