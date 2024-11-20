macro_rules! impl_froms {
    // Base case with no variants
    ($e:ident:) => {};

    // Match a boxed variant followed by more variants
    ($e:ident: box $v:ident, $($rest:tt)*) => {
        impl_froms!($e: box $v);
        impl_froms!($e: $($rest)*);
    };

    // Match a regular variant followed by more variants
    ($e:ident: $v:ident, $($rest:tt)*) => {
        impl_froms!($e: $v);
        impl_froms!($e: $($rest)*);
    };

    // Match a single boxed variant (end case)
    ($e:ident: box $v:ident) => {
        impl From<$v> for $e {
            fn from(it: $v) -> $e {
                $e::$v(Box::new(it))
            }
        }
    };

    // Match a single regular variant (end case)
    ($e:ident: $v:ident) => {
        impl From<$v> for $e {
            fn from(it: $v) -> $e {
                $e::$v(it)
            }
        }
    };
}

pub(crate) use impl_froms;
