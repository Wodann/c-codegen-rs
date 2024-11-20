use core::fmt;

use crate::CType;

#[derive(Clone, Debug)]
pub enum CValue {
    Array {
        values: Vec<CValue>,
        base_type: CType,
    },
    Char {
        value: char,
        signed: bool,
    },
    Enum {
        value: i32,
        name: String,
    },
    Float {
        value: f64,
        ty: CType,
    },
    IntegerSigned {
        value: i64,
        ty: CType,
    },
    IntegerUnsigned {
        value: u64,
        ty: CType,
    },
    Pointer {
        address: usize,
        base_type: CType,
    },
    String(String),
    Struct {
        fields: Vec<(String, CValue)>,
    },
}

impl CValue {
    pub const fn int(value: i64) -> Self {
        Self::IntegerSigned {
            value,
            ty: CType::int(),
        }
    }
}

impl fmt::Display for CValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CValue::Array { values, base_type } => {
                let vals = values
                    .iter()
                    .map(|v| format!("{}", v))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{}[] {{ {} }}", base_type, vals)
            }
            CValue::Char { value, signed } => {
                write!(f, "{value}")
            }
            CValue::Enum { value, name } => write!(f, "enum {} = {}", name, value),
            CValue::Float { value, ty } => write!(f, "{value}"),
            CValue::IntegerSigned { value, ty } => write!(f, "{value}"),
            CValue::IntegerUnsigned { value, ty } => write!(f, "{value}"),
            CValue::Pointer { address, base_type } => {
                write!(f, "{}* at 0x{:x}", base_type, address)
            }
            CValue::String(val) => write!(f, "\"{}\"", val),
            CValue::Struct { fields } => {
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
