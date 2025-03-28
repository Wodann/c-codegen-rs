use pretty::Pretty as _;

use crate::function::FunctionParameter;

use super::ConcreteType;

#[derive(Clone)]
pub struct Function {
    pub parameters: Vec<FunctionParameter>,
    pub return_ty: ConcreteType,
}

impl Function {
    pub fn pretty_return_type<'a, AllocatorT, AnnotationT>(
        &self,
        allocator: &'a AllocatorT,
    ) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT>
    where
        AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
        AllocatorT::Doc: Clone,
        AnnotationT: Clone + 'a,
    {
        allocator
            .text(self.return_ty.to_string())
            .append(allocator.space())
    }

    pub fn pretty_parameters<'a, AllocatorT, AnnotationT>(
        &self,
        allocator: &'a AllocatorT,
    ) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT>
    where
        AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
        AllocatorT::Doc: Clone,
        AnnotationT: Clone + 'a,
    {
        allocator
            .text("(")
            .append(
                allocator.intersperse(
                    self.parameters
                        .iter()
                        .map(|parameter| parameter.clone().pretty(allocator)),
                    allocator.text(",").append(allocator.space()),
                ),
            )
            .append(allocator.text(")"))
    }
}
