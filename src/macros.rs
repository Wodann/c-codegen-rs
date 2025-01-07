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

    // Match a nested variant with multiple inner types, the first being boxed, followed by more variants
    ($e:ident: $outer:ident(box $inner:ident, $($others:tt)*), $($rest:tt)*) => {
        impl_froms!($e: $outer(box $inner));
        impl_froms!($e: $outer($($others)*));
        impl_froms!($e: $($rest)*);
    };

    // Match a nested variant with multiple inner types, the first being regular, followed by more variants
    ($e:ident: $outer:ident($inner:ident, $($others:tt)*), $($rest:tt)*) => {
        impl_froms!($e: $outer($inner));
        impl_froms!($e: $outer($($others)*));
        impl_froms!($e: $($rest)*);
    };

    // Match a single nested variant with multiple inner types, the first being boxed (end case)
    ($e:ident: $outer:ident(box $inner:ident, $($others:tt)*)) => {
        impl_froms!($e: $outer(box $inner));
        impl_froms!($e: $outer($($others)*));
    };

    // Match a single nested variant with multiple inner types, the first being regular (end case)
    ($e:ident: $outer:ident($inner:ident, $($others:tt)*)) => {
        impl_froms!($e: $outer($inner));
        impl_froms!($e: $outer($($others)*));
    };

    // Match a single nested boxed variant with multiple inner types (end case)
    ($e:ident: $outer:ident(box $inner:ident)) => {
        impl From<$outer> for $e {
            fn from(it: $outer) -> $e {
                $e::$outer(it)
            }
        }

        impl From<$inner> for $e {
            fn from(it: $inner) -> $e {
                $e::$outer($outer::$inner(Box::new(it)))
            }
        }
    };

    // Match a single nested boxed variant with multiple inner types (end case)
    ($e:ident: $outer:ident($inner:ident)) => {
        impl From<$outer> for $e {
            fn from(it: $outer) -> $e {
                $e::$outer(it)
            }
        }

        impl From<$inner> for $e {
            fn from(it: $inner) -> $e {
                $e::$outer($outer::$inner(it))
            }
        }
    };
}

pub(crate) use impl_froms;
