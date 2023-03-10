use super::{ alternative, errors::*, monadic::* };

pub fn most_till<'a, Iter, Err, T, U>(
    parser: &'a impl Fn(&mut Iter) -> Result<T, Err>,
    end: &'a impl Fn(&mut Iter) -> Result<T, Err>
)
    -> impl Fn(&mut Iter) -> Result<(Vec<T>, U), Vec<Err>> + 'a
{
    |iter| alternative!(
        fmap2::<Iter, Vec<Err>, T, (Vec<T>, U), (Vec<T>, U)>(
            & |t, (mut ts, u)| { ts.insert(0, t); (ts, u) },
            & wrap_err(parser),
            & most_till(parser, end)
        )(iter)
    )
}