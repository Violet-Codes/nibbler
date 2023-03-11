use super::parser;

pub fn eos<Iter: Iterator, Err>(
    msg: impl Fn(&Iter) -> Err
)
    -> parser![Iter, Err, ()]
{
    move |iter| {
        let err = msg(iter);
        match iter.next() {
            Option::Some(_token) => Result::Err(err),
            Option::None => Result::Ok(())
        }
    }
}

pub fn expect<Iter: Iterator, Err, const N: usize>(
    a: [Iter::Item; N],
    msg: impl Fn(&Iter) -> Err
)
    -> parser![Iter, Err, [Iter::Item; N]]
where
    Iter::Item: PartialEq
{
    move |iter| {
        let err = msg(iter);
        match iter.next_chunk::<N>() {
            Result::Ok(b) => if a == b {
                Result::Ok(b)
            } else {
                Result::Err(err)
            },
            Result::Err(_into) => Result::Err(err)
        }
    }
}

pub fn predicate<Iter: Iterator, Err, const N: usize>(
    f: impl Fn(& [Iter::Item; N]) -> bool,
    msg: impl Fn(&Iter) -> Err
)
    -> parser![Iter, Err, [Iter::Item; N]]
{
    move |iter| {
        let err = msg(iter);
        match iter.next_chunk::<N>() {
            Result::Ok(a) => if f(& a) {
                Result::Ok(a)
            } else {
                Result::Err(err)
            },
            Result::Err(_into) => Result::Err(err)
        }
    }
}