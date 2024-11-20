use core::fmt;

#[derive(Clone)]
pub struct Integer {
    pub kind: IntegerKind,
    pub is_signed: bool,
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_signed {
            if matches!(self.kind, IntegerKind::Char) {
                write!(f, "signed char")
            } else {
                write!(f, "{}", self.kind)
            }
        } else {
            write!(f, "unsigned {}", self.kind)
        }
    }
}

/// # Source
///
/// https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Integer-Types
#[derive(Clone)]
pub enum IntegerKind {
    Char,
    Short,
    Int,
    Long,
    LongLong,
}

impl fmt::Display for IntegerKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IntegerKind::Char => write!(f, "char"),
            IntegerKind::Short => write!(f, "short"),
            IntegerKind::Int => write!(f, "int"),
            IntegerKind::Long => write!(f, "long"),
            IntegerKind::LongLong => write!(f, "long long"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum StrongInt {
    Int16,
    Int32,
    Int64,
    Int8,
    IntFast16,
    IntFast32,
    IntFast64,
    IntFast8,
    IntLeast16,
    IntLeast32,
    IntLeast64,
    IntLeast8,
    IntMax,
    IntPtr,
    Uint16,
    Uint32,
    Uint64,
    Uint8,
    UintFast16,
    UintFast32,
    UintFast64,
    UintFast8,
    UintLeast16,
    UintLeast32,
    UintLeast64,
    UintLeast8,
    UintMax,
    UintPtr,
}

impl fmt::Display for StrongInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StrongInt::Int16 => write!(f, "int16_t"),
            StrongInt::Int32 => write!(f, "int32_t"),
            StrongInt::Int64 => write!(f, "int64_t"),
            StrongInt::Int8 => write!(f, "int8_t"),
            StrongInt::IntFast16 => write!(f, "int_fast16_t"),
            StrongInt::IntFast32 => write!(f, "int_fast32_t"),
            StrongInt::IntFast64 => write!(f, "int_fast64_t"),
            StrongInt::IntFast8 => write!(f, "int_fast8_t"),
            StrongInt::IntLeast16 => write!(f, "int_least16_t"),
            StrongInt::IntLeast32 => write!(f, "int_least32_t"),
            StrongInt::IntLeast64 => write!(f, "int_least64_t"),
            StrongInt::IntLeast8 => write!(f, "int_least8_t"),
            StrongInt::IntMax => write!(f, "intmax_t"),
            StrongInt::IntPtr => write!(f, "intptr_t"),
            StrongInt::Uint16 => write!(f, "uint16_t"),
            StrongInt::Uint32 => write!(f, "uint32_t"),
            StrongInt::Uint64 => write!(f, "uint64_t"),
            StrongInt::Uint8 => write!(f, "uint8_t"),
            StrongInt::UintFast16 => write!(f, "uint_fast16_t"),
            StrongInt::UintFast32 => write!(f, "uint_fast32_t"),
            StrongInt::UintFast64 => write!(f, "uint_fast64_t"),
            StrongInt::UintFast8 => write!(f, "uint_fast8_t"),
            StrongInt::UintLeast16 => write!(f, "uint_least16_t"),
            StrongInt::UintLeast32 => write!(f, "uint_least32_t"),
            StrongInt::UintLeast64 => write!(f, "uint_least64_t"),
            StrongInt::UintLeast8 => write!(f, "uint_least8_t"),
            StrongInt::UintMax => write!(f, "uintmax_t"),
            StrongInt::UintPtr => write!(f, "uintptr_t"),
        }
    }
}
