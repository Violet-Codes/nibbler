mod monadic;
mod errors;
mod builders;
mod combinators;
mod utils;

// parser: impl Fn(&mut: Iter) -> Result<Res, Err>

#[macro_export]
macro_rules! alternative {
    ($x:expr) => ($x);
    ($x:expr, $($xs:expr), +) => (
        crate::monadic::otherwise($x, alternative!($($xs), +))
    )
}

#[macro_export]
macro_rules! first {
    ($x:expr) => ($x);
    ($x:expr, $($xs:expr), +) => (
        crate::monadic::fmap2(
            |x0, x1| x0,
            $x,
            first!($($xs), +)
        )
    )
}

#[macro_export]
macro_rules! last {
    ($x:expr) => ($x);
    ($x:expr, $($xs:expr), +) => (
        crate::monadic::fmap2(
            |x0, x1| x1,
            $x,
            last!($($xs), +)
        )
    )
}

#[macro_export]
macro_rules! select {
    ($($xs:expr), *, item $y:expr, $($zs:expr), *) => (
        first!(last!($($xs), *, $y), $($zs), *)
    )
}