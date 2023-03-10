mod monadic;
mod errors;
mod builders;
mod combinators;
mod utils;

// parser: impl Fn(&mut: Iter) -> Result<Res, Err>


#[macro_export]
macro_rules! alternative {
    ($x:expr) => ($x);
    ($x:expr, $($xs:expr),+) => (
        monadic::otherwise($x, alternative!($($xs),+))
    )
}