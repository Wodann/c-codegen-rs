use crate::{identifier, non_empty_vec};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Identifier(#[from] identifier::Error),
    #[error(transparent)]
    NonEmptyVec(#[from] non_empty_vec::Error),
}
