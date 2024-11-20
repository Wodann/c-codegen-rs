use core::fmt;
use std::io;

use crate::{function, variable};

enum FileLevelStatement {
    FunctionDeclaration(function::Declaration),
    FunctionDefinition(function::Definition),
    Variable(variable::Declaration),
}

impl From<function::Declaration> for FileLevelStatement {
    fn from(declaration: function::Declaration) -> Self {
        FileLevelStatement::FunctionDeclaration(declaration)
    }
}

impl From<function::Definition> for FileLevelStatement {
    fn from(definition: function::Definition) -> Self {
        FileLevelStatement::FunctionDefinition(definition)
    }
}

impl From<variable::Declaration> for FileLevelStatement {
    fn from(declaration: variable::Declaration) -> Self {
        FileLevelStatement::Variable(declaration)
    }
}

impl fmt::Display for FileLevelStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileLevelStatement::FunctionDeclaration(declaration) => write!(f, "{declaration}"),
            FileLevelStatement::FunctionDefinition(definition) => write!(f, "{definition}"),
            FileLevelStatement::Variable(declaration) => write!(f, "{declaration}"),
        }
    }
}

#[derive(Default)]
pub struct CFileBuilder {
    statements: Vec<FileLevelStatement>,
}

impl CFileBuilder {
    /// Adds a [`function::Declaration`] to the file.
    pub fn add_function_declaration(&mut self, declaration: function::Declaration) -> &mut Self {
        self.statements.push(declaration.into());

        self
    }

    /// Adds a [`function::Definition`] to the file.
    pub fn add_function_definition(&mut self, definition: function::Definition) -> &mut Self {
        self.statements.push(definition.into());

        self
    }

    /// Adds a [`variable::Declaration`] to the file.
    pub fn add_variable_declaration(&mut self, declaration: variable::Declaration) -> &mut Self {
        self.statements.push(declaration.into());

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
