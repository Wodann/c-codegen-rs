use core::fmt;
use std::io;

use crate::{function, r#type, variable};

pub enum FileLevelStatement {
    FunctionDeclaration(function::Declaration),
    FunctionDefinition(function::Definition),
    TypeDefinition(r#type::Definition),
    Variable(variable::Declaration),
}

impl From<function::Declaration> for FileLevelStatement {
    fn from(value: function::Declaration) -> Self {
        FileLevelStatement::FunctionDeclaration(value)
    }
}

impl From<function::Definition> for FileLevelStatement {
    fn from(value: function::Definition) -> Self {
        FileLevelStatement::FunctionDefinition(value)
    }
}

impl From<r#type::Definition> for FileLevelStatement {
    fn from(value: r#type::Definition) -> Self {
        FileLevelStatement::TypeDefinition(value)
    }
}

impl From<variable::Declaration> for FileLevelStatement {
    fn from(value: variable::Declaration) -> Self {
        FileLevelStatement::Variable(value)
    }
}

impl fmt::Display for FileLevelStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileLevelStatement::FunctionDeclaration(declaration) => write!(f, "{declaration}"),
            FileLevelStatement::FunctionDefinition(definition) => write!(f, "{definition}"),
            FileLevelStatement::TypeDefinition(definition) => write!(f, "{definition}"),
            FileLevelStatement::Variable(declaration) => write!(f, "{declaration}"),
        }
    }
}

#[derive(Default)]
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
