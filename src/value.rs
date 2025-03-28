use core::fmt;

use crate::{
    r#type::{IntegerKind, Real},
    ConcreteType,
};

#[derive(Clone)]
pub enum Value {
    Array {
        values: Vec<Value>,
        base_type: ConcreteType,
    },
    Char {
        value: char,
    },
    Enum {
        value: i32,
        name: String,
    },
    IntegerSigned {
        value: i64,
        kind: IntegerKind,
    },
    IntegerUnsigned {
        value: u64,
        kind: IntegerKind,
    },
    Pointer {
        address: usize,
    },
    Real {
        value: f64,
        kind: Real,
    },
    Size {
        value: usize,
    },
    String(String),
    Struct {
        fields: Vec<(String, Value)>,
    },
}

impl Value {
    pub const fn float(value: f64) -> Self {
        Self::Real {
            value,
            kind: Real::Float,
        }
    }

    pub const fn int(value: i64) -> Self {
        Self::IntegerSigned {
            value,
            kind: IntegerKind::Int,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Array { values, base_type } => {
                let vals = values
                    .iter()
                    .map(|v| format!("{}", v))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{}[] {{ {} }}", base_type, vals)
            }
            Value::Char { value } => {
                write!(f, "'{value}'")
            }
            Value::Enum { value, name } => write!(f, "enum {} = {}", name, value),
            Value::Real { value, kind: ty } => write!(f, "{value}"),
            Value::IntegerSigned { value, kind: ty } => write!(f, "{value}"),
            Value::IntegerUnsigned { value, kind: ty } => write!(f, "{value}"),
            Value::Pointer { address } => {
                write!(f, "{address:#x}")
            }
            Value::Size { value } => write!(f, "{value}"),
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
