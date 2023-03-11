use super::{ alternative, errors::*, monadic::* };

pub fn most_till<Iter, Err, T, U>(
    parser: impl Fn(&mut Iter) -> Result<T, Err>,
    end: impl Fn(&mut Iter) -> Result<U, Err>
)
    -> impl Fn(&mut Iter) -> Result<(Vec<T>, U), Vec<Err>>
{
    move |iter| alternative! (
        fmap2(
            |t, (mut ts, u) | { ts.insert(0, t); (ts, u) },
            wrap_err(& parser),
            most_till(& parser, & end)
        ),
        fmap(
            |u| (vec![], u),
            wrap_err(& end)
        )
    )(iter)
}

pub fn least_till<Iter, Err, T, U>(
    parser: impl Fn(&mut Iter) -> Result<T, Err>,
    end: impl Fn(&mut Iter) -> Result<U, Err>
)
    -> impl Fn(&mut Iter) -> Result<(Vec<T>, U), Vec<Err>>
{
    move |iter| alternative! (
        fmap(
            |u| (vec![], u),
            wrap_err(& end)
        ),
        fmap2(
            |t, (mut ts, u) | { ts.insert(0, t); (ts, u) },
            wrap_err(& parser),
            least_till(& parser, & end)
        )
    )(iter)
}

pub fn most<Iter, Err, T>(
    parser: impl Fn(&mut Iter) -> Result<T, Err>
)
    -> impl Fn(&mut Iter) -> Result<Vec<T>, Err>
{
    move |iter| {
        let mut ts: Vec<T> = vec![];
        while let Result::Ok(t) = parser(iter) {
            ts.push(t);
        }
        return Result::Ok(ts);
    }
}

pub fn unwind<Iter: Clone, Err, T>(
    parser: impl Fn(&mut Iter) -> Result<T, Err>
)
    -> impl Fn(&mut Iter) -> Result<T, Err>
{
    move |iter| {
        let pre: Iter = iter.clone();
        let res = parser(iter);
        *iter = pre;
        res
    }
}