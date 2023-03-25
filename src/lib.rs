#![feature(iter_next_chunk)]
#![feature(decl_macro)]

pub mod monadic;
pub mod errors;
pub mod builders;
pub mod combinators;
pub mod utils;
pub mod state;
pub mod text;

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

#[cfg(test)]
mod tests {
    use crate::{*, state::CountIter};

    macro_rules! count {
        ($iter:expr) => (state::CountIter{
            iter: $iter,
            index: 0
        });
    }

    macro_rules! msg {
        () => (
            |iter_: & CountIter<std::str::Chars>| iter_.index
        );
    }

    #[test]
    fn parse_pure() {
        let mut iter = count!("-".chars());
        let res: Result<(), usize> = monadic::pure(|| ())(&mut iter);

        assert_eq!(Ok(()), res); // expect success

        assert_eq!(Some('-'), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn parse_eos_on_empty() {
        let mut iter = count!("".chars());
        let res: Result<(), usize> = builders::eos(msg!())(&mut iter);

        assert_eq!(Ok(()), res); // expect success

        assert_eq!(None, iter.next());
    }

    #[test]
    fn parse_eos_on_content() {
        let mut iter = count!("a-".chars());
        let res: Result<(), usize> = builders::eos(msg!())(&mut iter);

        assert_eq!(Err(0), res); // expect failure

        assert_eq!(Some('-'), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn parse_a_on_a() {
        let mut iter = count!("a-".chars());
        let res: Result<[char; 1], usize> = builders::expect(['a'], msg!())(&mut iter);

        assert_eq!(Ok(['a']), res); // expect success

        assert_eq!(Some('-'), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn parse_unwind_a_on_a() {
        let mut iter = count!("a-".chars());
        let res: Result<[char; 1], usize> = combinators::unwind(builders::expect(['a'], msg!()))(&mut iter);

        assert_eq!(Ok(['a']), res); // expect success

        assert_eq!(Some('a'), iter.next());
        assert_eq!(Some('-'), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn parse_a_on_b() {
        let mut iter = count!("b-".chars());
        let res: Result<[char; 1], usize> = builders::expect(['a'], msg!())(&mut iter);

        assert_eq!(Err(0), res); // expect failure

        assert_eq!(Some('-'), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn parse_try_a_on_b() {
        let mut iter = count!("b-".chars());
        let res: Result<[char; 1], usize> = errors::try_parse(builders::expect(['a'], msg!()))(&mut iter);

        assert_eq!(Err(0), res); // expect failure

        assert_eq!(Some('b'), iter.next());
        assert_eq!(Some('-'), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn parse_unwind_a_on_b() {
        let mut iter = count!("b-".chars());
        let res: Result<[char; 1], usize> = combinators::unwind(builders::expect(['a'], msg!()))(&mut iter);

        assert_eq!(Err(0), res); // expect failure

        assert_eq!(Some('b'), iter.next());
        assert_eq!(Some('-'), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn parse_abc_on_abc() {
        let mut iter = count!("abc-".chars());
        let res: Result<[char; 3], usize> = builders::expect(['a', 'b', 'c'], msg!())(&mut iter);

        assert_eq!(Ok(['a', 'b', 'c']), res); // expect success

        assert_eq!(Some('-'), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn parse_abc_on_acd() {
        let mut iter = count!("acd-".chars());
        let res: Result<[char; 3], usize> = builders::expect(['a', 'b', 'c'], msg!())(&mut iter);

        assert_eq!(Err(0), res); // expect failure

        assert_eq!(Some('-'), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn parse_a_b_c_on_abc() {
        let mut iter = count!("abc-".chars());
        let res: Result<[char; 1], usize> = select!(
            builders::expect(['a'], msg!()),
            => builders::expect(['b'], msg!()),
            builders::expect(['c'], msg!())
        )(&mut iter);

        assert_eq!(Ok(['b']), res); // expect success

        assert_eq!(Some('-'), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn parse_a_b_c_on_acd() {
        let mut iter = count!("acd-".chars());
        let res: Result<[char; 1], usize> = select!(
            builders::expect(['a'], msg!()),
            => builders::expect(['b'], msg!()),
            builders::expect(['c'], msg!())
        )(&mut iter);

        assert_eq!(Err(1), res); // expect failure

        assert_eq!(Some('d'), iter.next());
        assert_eq!(Some('-'), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn parse_a_try_b_c_on_acd() {
        let mut iter = count!("acd-".chars());
        let res: Result<[char; 1], usize> = select!(
            builders::expect(['a'], msg!()),
            => errors::try_parse(builders::expect(['b'], msg!())),
            builders::expect(['c'], msg!())
        )(&mut iter);

        assert_eq!(Err(1), res); // expect failure

        assert_eq!(Some('c'), iter.next());
        assert_eq!(Some('d'), iter.next());
        assert_eq!(Some('-'), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn parse_tryall_a_b_c_on_acd() {
        let mut iter = count!("acd-".chars());
        let res: Result<[char; 1], usize> = errors::try_parse(select!(
            builders::expect(['a'], msg!()),
            => builders::expect(['b'], msg!()),
            builders::expect(['c'], msg!())
        ))(&mut iter);

        assert_eq!(Err(1), res); // expect failure

        assert_eq!(Some('a'), iter.next());
        assert_eq!(Some('c'), iter.next());
        assert_eq!(Some('d'), iter.next());
        assert_eq!(Some('-'), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn parse_most_a_on_b() {
        let mut iter = count!("b-".chars());
        let res: Result<Vec<[char; 1]>, usize> = combinators::most(builders::expect(['a'], msg!()))(&mut iter);

        assert_eq!(Ok(vec![]), res); // expect success

        assert_eq!(Some('-'), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn parse_most_try_a_on_b() {
        let mut iter = count!("b-".chars());
        let res: Result<Vec<[char; 1]>, usize> = combinators::most(errors::try_parse(builders::expect(['a'], msg!())))(&mut iter);

        assert_eq!(Ok(vec![]), res); // expect success

        assert_eq!(Some('b'), iter.next());
        assert_eq!(Some('-'), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn parse_most_a_on_aab() {
        let mut iter = count!("aab-".chars());
        let res: Result<Vec<[char; 1]>, usize> = combinators::most(builders::expect(['a'], msg!()))(&mut iter);

        assert_eq!(Ok(vec![['a'], ['a']]), res); // expect success

        assert_eq!(Some('-'), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn parse_most_try_a_on_aab() {
        let mut iter = count!("aab-".chars());
        let res: Result<Vec<[char; 1]>, usize> = combinators::most(errors::try_parse(builders::expect(['a'], msg!())))(&mut iter);

        assert_eq!(Ok(vec![['a'], ['a']]), res); // expect success

        assert_eq!(Some('b'), iter.next());
        assert_eq!(Some('-'), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn parse_most_a_till_c_on_abc() {
        let mut iter = count!("abc-".chars());
        let res: Result<(Vec<[char; 1]>, [char; 1]), Vec<usize>> = combinators::most_till(
            builders::expect(['a'], msg!()),
            builders::expect(['c'], msg!())
        )(&mut iter);

        assert_eq!(Ok((vec![['a']], ['c'])), res); // expect success

        assert_eq!(Some('-'), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn parse_most_try_a_till_b_on_aab() {
        let mut iter = count!("aab-".chars());
        let res: Result<(Vec<[char; 1]>, [char; 1]), Vec<usize>> = combinators::most_till(
            errors::try_parse(builders::expect(['a'], msg!())),
            builders::expect(['b'], msg!())
        )(&mut iter);

        assert_eq!(Ok((vec![['a'], ['a']], ['b'])), res); // expect success

        assert_eq!(Some('-'), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn parse_least_b_till_c_on_abc() {
        let mut iter = count!("abc-".chars());
        let res: Result<(Vec<[char; 1]>, [char; 1]), Vec<usize>> = combinators::least_till(
            builders::expect(['b'], msg!()),
            builders::expect(['c'], msg!())
        )(&mut iter);

        assert_eq!(Ok((vec![['b']], ['c'])), res); // expect success

        assert_eq!(Some('-'), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn parse_least_a_till_try_b_on_aab() {
        let mut iter = count!("aab-".chars());
        let res: Result<(Vec<[char; 1]>, [char; 1]), Vec<usize>> = combinators::least_till(
            builders::expect(['a'], msg!()),
            errors::try_parse(builders::expect(['b'], msg!()))
        )(&mut iter);

        assert_eq!(Ok((vec![['a'], ['a']], ['b'])), res); // expect success

        assert_eq!(Some('-'), iter.next());
        assert_eq!(None, iter.next());
    }
}