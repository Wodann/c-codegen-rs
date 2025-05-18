use core::fmt;
use std::io;

use crate::{
    macros::impl_froms, r#type::Declaration as TypeDeclaration,
    r#type::Definition as TypeDefinition, statement::Include, FunctionDeclaration,
    FunctionDefinition, VariableDeclaration,
};

#[derive(Clone, Debug)]
pub enum FileLevelStatement {
    FunctionDeclaration(FunctionDeclaration),
    FunctionDefinition(FunctionDefinition),
    Include(Include),
    TypeDeclaration(TypeDeclaration),
    TypeDefinition(TypeDefinition),
    VariableDeclaration(VariableDeclaration),
}

impl_froms!(FileLevelStatement: FunctionDeclaration, FunctionDefinition, Include, TypeDefinition, VariableDeclaration);

impl fmt::Display for FileLevelStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileLevelStatement::FunctionDeclaration(declaration) => write!(f, "{declaration}"),
            FileLevelStatement::FunctionDefinition(definition) => write!(f, "{definition}"),
            FileLevelStatement::Include(include) => write!(f, "{include}"),
            FileLevelStatement::TypeDeclaration(declaration) => write!(f, "{declaration}"),
            FileLevelStatement::TypeDefinition(definition) => write!(f, "{definition}"),
            FileLevelStatement::VariableDeclaration(declaration) => write!(f, "{declaration}"),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct CFileBuilder {
    statements: Vec<FileLevelStatement>,
}

impl CFileBuilder {
    /// Adds a [`TopLevelStatement`] to the file.
    pub fn add_statement<T: Into<FileLevelStatement>>(&mut self, statement: T) -> &mut Self {
        self.statements.push(statement.into());

        self
    }

    /// Generates the C code and writes it to the given [`Write`] instance using UTF-8 encoding.
    pub fn generate<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        for declaration in &self.statements {
            writeln!(writer, "{declaration}")?;
        }

        Ok(())
    }

    /// Generates the C code and writes it to the given file path.
    pub fn write_to_file(&self, file_path: &str) -> io::Result<()> {
        let mut file = std::fs::File::create(file_path)?;
        self.generate(&mut file)
    }

    pub fn write_to_string(&self) -> io::Result<String> {
        let mut buffer = Vec::new();
        self.generate(&mut buffer)?;

        // Safety: [`generate`] only writes valid UTF-8 data.
        let string = unsafe { String::from_utf8_unchecked(buffer) };

        Ok(string)
    }
}
