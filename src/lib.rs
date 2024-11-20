mod block;
mod expression;
mod file;
pub mod function;
pub mod pretty;
mod statement;
mod storage_class;
pub mod r#type;
pub mod value;
pub mod variable;

pub use self::{
    block::Block, expression::Expression, file::CFileBuilder, r#type::CType, statement::CStatement,
    storage_class::StorageClass, value::Value,
};

pub type Identifier = String;
