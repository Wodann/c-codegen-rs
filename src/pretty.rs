macro_rules! impl_display_via_pretty {
    ($type:ty, $text_width:literal) => {
        impl core::fmt::Display for $type {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                use pretty::Pretty as _;

                let output = {
                    let allocator = pretty::Arena::<'_, ()>::new();
                    let doc = self.clone().pretty(&allocator);

                    let mut output = String::new();
                    doc.render_fmt($text_width, &mut output)
                        .map_err(|_| core::fmt::Error)?;

                    output
                };

                write!(f, "{}", output)
            }
        }
    };
}

pub(crate) use impl_display_via_pretty;
