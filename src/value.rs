use core::fmt;

use crate::{
    r#type::{IntegerKind, Real},
    ConcreteType,
};

#[derive(Clone, Debug)]
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
    Pointer {
        address: usize,
    },
    Real {
        value: f64,
        kind: Option<Real>,
    },
    SignedInteger {
        value: i64,
        kind: Option<IntegerKind>,
    },
    Size {
        value: usize,
    },
    String(String),
    Struct {
        fields: Vec<(String, Value)>,
    },
    UnsignedInteger {
        value: u64,
        kind: Option<IntegerKind>,
    },
}

impl Value {
    pub const fn double(value: f64) -> Self {
        Self::Real {
            value,
            kind: Some(Real::Double),
        }
    }

    pub const fn float(value: f64) -> Self {
        Self::Real {
            value,
            kind: Some(Real::Float),
        }
    }

    pub const fn int(value: i64) -> Self {
        Self::SignedInteger {
            value,
            kind: Some(IntegerKind::Int),
        }
    }

    pub const fn long_double(value: f64) -> Self {
        Self::Real {
            value,
            kind: Some(Real::LongDouble),
        }
    }

    pub const fn real(value: f64) -> Self {
        Self::Real { value, kind: None }
    }

    pub const fn signed_integer(value: i64) -> Self {
        Self::SignedInteger { value, kind: None }
    }

    pub const fn unsigned_integer(value: u64) -> Self {
        Self::UnsignedInteger { value, kind: None }
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
            Value::Real { value, kind } => {
                write!(f, "{value}{suffix}", suffix = kind.map_or("", Real::suffix))
            }
            Value::Pointer { address } => {
                write!(f, "{address:#x}")
            }
            Value::SignedInteger { value, kind } => {
                write!(
                    f,
                    "{value}{suffix}",
                    suffix = kind.map_or("", IntegerKind::suffix)
                )
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
            Value::UnsignedInteger { value, kind } => {
                write!(
                    f,
                    "{value}u{suffix}",
                    suffix = kind.map_or("", IntegerKind::suffix)
                )
            }
        }
    }
}
