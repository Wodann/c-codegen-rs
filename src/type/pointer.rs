use core::fmt;

use pretty::Pretty;

use crate::{pretty::impl_display_via_pretty, Type};

#[derive(Clone)]
pub struct Pointer {
    pub pointer_ty: Box<Type>,
    pub is_const: bool,
}

impl Pointer {
    pub fn base_type(&self) -> Type {
        // TODO: Handle array
        match self.pointer_ty.as_ref() {
            Type::Pointer(pointer) => pointer.base_type(),
            ty => ty.clone(),
        }
    }
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Pointer
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        if let Type::Function(function) = self.base_type() {
            let return_type = function.pretty_return_type(allocator);
            let parameters = function.pretty_parameters(allocator);
        } else {
            if self.is_const {
                write!(f, "{}* const", self.pointer_ty)
            } else {
                write!(f, "{}*", self.pointer_ty)
            }
        }
    }
}

impl_display_via_pretty!(Pointer, 80);
