#![feature(iter_next_chunk)]

pub mod monadic;
pub mod errors;
pub mod builders;
pub mod combinators;
pub mod utils;
pub mod state;

// parser: impl Fn(&mut: Iter) -> Result<Res, Err>

#[macro_export]
macro_rules! parser {
    ($iter:ty, $err:ty, $t:ty) => (impl Fn(&mut $iter) -> Result<$t, $err>)
}

// <|>
#[macro_export]
macro_rules! alternative {
    ($x:expr) => ($x);
    ($x:expr, $($xs:expr), +) => (
        crate::monadic::otherwise($x, alternative!($($xs), +))
    )
}

// <*
#[macro_export]
macro_rules! first {
    ($x:expr) => ($x);
    ($x:expr, $($xs:expr), +) => (
        crate::monadic::fmap2(
            |x0, _x1| x0,
            $x,
            first!($($xs), +)
        )
    )
}

// *>
#[macro_export]
macro_rules! last {
    ($x:expr) => ($x);
    ($x:expr, $($xs:expr), +) => (
        crate::monadic::fmap2(
            |_x0, x1| x1,
            $x,
            last!($($xs), +)
        )
    )
}

// *> <*
#[macro_export]
macro_rules! select {
    ($($xs:expr), *, => $y:expr, $($zs:expr), *) => (
        first!(last!($($xs), *, $y), $($zs), *)
    )
}
