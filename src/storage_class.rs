use core::fmt;

/// # Source
///
/// https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Storage-Class-Specifiers
#[derive(Clone)]
pub enum StorageClass {
    Auto,
    Extern,
    Register,
    Static,
}

impl fmt::Display for StorageClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StorageClass::Auto => write!(f, "auto"),
            StorageClass::Extern => write!(f, "extern"),
            StorageClass::Register => write!(f, "register"),
            StorageClass::Static => write!(f, "static"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{operator, variable, Type, Expression, Value};

    use super::*;

    // Source:
    // https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Storage-Class-Specifiers
    #[test]
    fn with_storage_class_extern() -> anyhow::Result<()> {
        let generated = variable::Declaration {
            storage_class: Some(StorageClass::Extern),
            ty: Type::int(),
            initializers: vec![variable::Initializer::Nil {
                variable_name: "numberOfClients".to_string(),
            }],
        }
        .to_string();

        assert_eq!(generated, "extern int numberOfClients;");

        let generated = variable::Declaration {
            storage_class: None,
            ty: Type::int(),
            initializers: vec![variable::Initializer::Assignment(operator::Assignment {
                variable_name: "numberOfClients".to_string(),
                expression: Expression::Value(Value::int(0)),
            })],
        }
        .to_string();

        assert_eq!(generated, "int numberOfClients = 0;");

        Ok(())
    }
}
