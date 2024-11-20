use core::fmt;

/// # Source
///
/// https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Storage-Class-Specifiers
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
    use crate::{variable, CType, CValue};

    use super::*;

    // Source:
    // https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Storage-Class-Specifiers
    #[test]
    fn with_storage_class_extern() -> anyhow::Result<()> {
        let generated = variable::Declaration {
            name: "numberOfClients".to_string(),
            ty: CType::int(),
            initializer: None,
            storage_class: Some(StorageClass::Extern),
        }
        .to_string();

        assert_eq!(generated, "extern int numberOfClients;");

        let generated = variable::Declaration {
            name: "numberOfClients".to_string(),
            ty: CType::int(),
            initializer: Some(CValue::int(0)),
            storage_class: None,
        }
        .to_string();

        assert_eq!(generated, "int numberOfClients = 0;");

        Ok(())
    }
}
