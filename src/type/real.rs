use std::fmt;

/// # Source
///
/// https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Real-Number-Types
#[derive(Clone, Debug)]
pub enum RealType {
    Float,
    Double,
    LongDouble,
}

impl fmt::Display for RealType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RealType::Float => write!(f, "float"),
            RealType::Double => write!(f, "double"),
            RealType::LongDouble => write!(f, "long double"),
        }
    }
}
