mod block;
mod error;
mod expression;
mod file;
pub mod function;
pub mod identifier;
mod macros;
pub mod non_empty_vec;
pub mod operator;
pub mod pretty;
pub mod statement;
mod storage_class;
pub mod r#type;
pub mod value;
pub mod variable;

pub use self::{
    block::Block, error::Error, expression::Expression, file::CFileBuilder,
    function::Declaration as FunctionDeclaration, function::Definition as FunctionDefinition,
    identifier::Identifier, r#type::ConcreteType, statement::Statement,
    storage_class::StorageClass, value::Value, variable::Declaration as VariableDeclaration,
    variable::Variable,
};

pub type Result<T> = std::result::Result<T, Error>;
