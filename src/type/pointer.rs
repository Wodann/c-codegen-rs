use pretty::Pretty;

use crate::{pretty::impl_display_via_pretty, ConcreteType};

use super::OpaqueType;

#[derive(Clone)]
pub struct Pointer {
    pub pointer_ty: OpaqueType,
    pub is_const: bool,
}

impl Pointer {
    pub fn base_type(&self) -> OpaqueType {
        // TODO: Handle array
        match &self.pointer_ty {
            OpaqueType::ConcreteType(ConcreteType::Pointer(pointer)) => pointer.base_type(),
            ty => ty.clone(),
        }
    }

    /// Flattens the constness of the pointer chain.
    pub fn flatten_pointers(&self) -> Vec<bool> {
        let mut flattened = vec![self.is_const];

        if let OpaqueType::ConcreteType(ConcreteType::Pointer(pointer)) = &self.pointer_ty {
            flattened.extend(pointer.flatten_pointers());
        }

        flattened
    }

    pub fn pretty_pointers<'a, AllocatorT, AnnotationT>(
        &self,
        allocator: &'a AllocatorT,
    ) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT>
    where
        AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
        AllocatorT::Doc: Clone,
        AnnotationT: Clone + 'a,
    {
        let mut builder = allocator.nil();

        let mut needs_space = false;
        for is_const in self.flatten_pointers() {
            if needs_space {
                builder = builder.append(allocator.space());
                needs_space = false;
            }

            builder = builder.append(allocator.text("*"));

            if is_const {
                builder = builder.append(allocator.text("const"));
                needs_space = true;
            }
        }

        builder
    }
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Pointer
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        match self.base_type() {
            OpaqueType::ConcreteType(base_type) => base_type
                .pretty(allocator)
                .append(allocator.space())
                .append(self.pretty_pointers(allocator)),
            OpaqueType::Function(function) => {
                let return_type = function.pretty_return_type(allocator);
                let parameters = function.pretty_parameters(allocator);

                return_type
                    .append(allocator.space())
                    .append(allocator.text("("))
                    .append(self.pretty_pointers(allocator))
                    .append(allocator.text(")"))
                    .append(parameters)
            }
        }
    }
}

impl_display_via_pretty!(Pointer, 80);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn const_pointer() -> anyhow::Result<()> {
        let pointer = Pointer {
            pointer_ty: ConcreteType::Void.into(),
            is_const: true,
        }
        .to_string();
        assert_eq!(pointer, "void *const");

        Ok(())
    }

    #[test]
    fn const_pointer_to_pointer() -> anyhow::Result<()> {
        let pointer = Pointer {
            pointer_ty: Pointer {
                pointer_ty: ConcreteType::Void.into(),
                is_const: false,
            }
            .into(),
            is_const: true,
        }
        .to_string();
        assert_eq!(pointer, "void *const *");

        Ok(())
    }

    #[test]
    fn pointer() -> anyhow::Result<()> {
        let pointer = Pointer {
            pointer_ty: ConcreteType::Void.into(),
            is_const: false,
        }
        .to_string();
        assert_eq!(pointer, "void *");

        Ok(())
    }
}
