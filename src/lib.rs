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
    ($x:expr) => ($x),
    ($x:expr, $($xs:expr), +) => (
        $crate::monadic::otherwise($x, alternative!($($xs), +))
    )
}

// <*
pub macro first {
    ($x:expr) => ($x),
    ($x:expr, $($xs:expr), +) => (
        $crate::monadic::fmap2(
            |x0, _x1| x0,
            $x,
            first!($($xs), +)
        )
    )
}

// *>
pub macro last {
    ($x:expr) => ($x),
    ($x:expr, $($xs:expr), +) => (
        $crate::monadic::fmap2(
            |_x0, x1| x1,
            $x,
            last!($($xs), +)
        )
    )
}

// *> <*
pub macro select {
    ($($xs:expr), *, => $y:expr, $($zs:expr), *) => (
        first!(last!($($xs), *, $y), $($zs), *)
    )
}
