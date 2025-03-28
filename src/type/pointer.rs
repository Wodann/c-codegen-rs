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
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Pointer
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        match self.base_type() {
            OpaqueType::ConcreteType(base_type) => {
                let mut builder = base_type.pretty(allocator);
                for is_const in self.flatten_pointers() {
                    builder = builder
                        .append(allocator.space())
                        .append(allocator.text("*"));

                    if is_const {
                        builder = builder.append(allocator.space().append(allocator.text("const")));
                    }
                }

                builder
            }
            OpaqueType::Function(function) => {
                let return_type = function.pretty_return_type(allocator);
                let parameters = function.pretty_parameters(allocator);

                let mut builder = return_type
                    .append(allocator.space())
                    .append(allocator.text("("));

                for is_const in self.flatten_pointers() {
                    builder = builder.append(allocator.text("*"));

                    if is_const {
                        builder = builder.append(allocator.space().append(allocator.text("const")));
                    }
                }

                builder.append(allocator.text(")")).append(parameters)
            }
        }
    }
}

impl_display_via_pretty!(Pointer, 80);
