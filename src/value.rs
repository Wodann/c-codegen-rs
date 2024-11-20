use core::fmt;

use crate::Type;

#[derive(Clone)]
pub enum Value {
    Array { values: Vec<Value>, base_type: Type },
    Char { value: char, signed: bool },
    Enum { value: i32, name: String },
    Float { value: f64, ty: Type },
    IntegerSigned { value: i64, ty: Type },
    IntegerUnsigned { value: u64, ty: Type },
    Pointer { address: usize, base_type: Type },
    String(String),
    Struct { fields: Vec<(String, Value)> },
}

impl Value {
    pub const fn int(value: i64) -> Self {
        Self::IntegerSigned {
            value,
            ty: Type::int(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Array { values, base_type } => {
                let vals = values
                    .iter()
                    .map(|v| format!("{}", v))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{}[] {{ {} }}", base_type, vals)
            }
            Value::Char { value, signed } => {
                write!(f, "{value}")
            }
            Value::Enum { value, name } => write!(f, "enum {} = {}", name, value),
            Value::Float { value, ty } => write!(f, "{value}"),
            Value::IntegerSigned { value, ty } => write!(f, "{value}"),
            Value::IntegerUnsigned { value, ty } => write!(f, "{value}"),
            Value::Pointer { address, base_type } => {
                write!(f, "{}* at 0x{:x}", base_type, address)
            }
            Value::String(val) => write!(f, "\"{}\"", val),
            Value::Struct { fields } => {
                let field_str = fields
                    .iter()
                    .map(|(name, value)| format!("{}: {}", name, value))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{{ {} }}", field_str)
            }
        }
    }
}
