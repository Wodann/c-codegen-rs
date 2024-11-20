use core::fmt;

use crate::Type;

#[derive(Clone)]
pub struct Pointer {
    pub pointer_ty: Box<Type>,
    pub is_const: bool,
}

impl fmt::Display for Pointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_const {
            write!(f, "{}* const", self.pointer_ty)
        } else {
            write!(f, "{}*", self.pointer_ty)
        }
    }
}
