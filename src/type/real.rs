use std::fmt;

/// # Source
///
/// https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Real-Number-Types
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Real {
    Float,
    Double,
    LongDouble,
}

impl fmt::Display for Real {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Real::Float => write!(f, "float"),
            Real::Double => write!(f, "double"),
            Real::LongDouble => write!(f, "long double"),
        }
    }
}
