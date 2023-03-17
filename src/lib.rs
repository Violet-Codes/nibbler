#![feature(iter_next_chunk)]
#![feature(decl_macro)]

pub mod monadic;
pub mod errors;
pub mod builders;
pub mod combinators;
pub mod utils;
pub mod state;

// parser: impl Fn(&mut: Iter) -> Result<Res, Err>

pub macro parser {
    ($iter:ty, $err:ty, $t:ty) => (impl Fn(&mut $iter) -> Result<$t, $err>)
}

// <|>
pub macro alternative {
    ($x:expr) => ($crate::errors::wrap_err($x)),
    ($x:expr, $($xs:expr),+) => (
        $crate::monadic::otherwise($crate::errors::wrap_err($x), $crate::alternative!($($xs),+))
    )
}

// <*
pub macro first {
    ($x:expr) => ($x),
    ($x:expr, $($xs:expr),+) => (
        $crate::monadic::fmap2(
            |x0, _x1| x0,
            $x,
            $crate::first!($($xs),+)
        )
    )
}

// *>
pub macro last {
    ($x:expr) => ($x),
    ($x:expr, $($xs:expr),+) => (
        $crate::monadic::fmap2(
            |_x0, x1| x1,
            $x,
            $crate::last!($($xs),+)
        )
    )
}

// *> <*
pub macro select {
    ($($xs:expr),+, => $y:expr, $($zs:expr),+) => (
        $crate::first!($crate::last!($($xs),+, $y), $($zs),+)
    ),
    ($($xs:expr),+, => $y:expr) => (
        $crate::last!($($xs),+, $y)
    ),
    (=> $y:expr, $($zs:expr),+) => (
        $crate::first!($y, $($zs),+)
    ),
    (=> $y:expr) => (
        ($y)
    )
}