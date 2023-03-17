use super::{ parser, errors::*, monadic::* };

pub const fn most_till<Iter, Err, T, U>(
    parser: parser![Iter, Err, T],
    end: parser![Iter, Err, U]
)
    -> parser![Iter, Vec<Err>, (Vec<T>, U)]
{
    move |iter| otherwise(
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

pub const fn least_till<Iter, Err, T, U>(
    parser: parser![Iter, Err, T],
    end: parser![Iter, Err, U]
)
    -> parser![Iter, Vec<Err>, (Vec<T>, U)]
{
    move |iter| otherwise(
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

pub const fn most<Iter, Err, T>(
    parser: parser![Iter, Err, T]
)
    -> parser![Iter, Err, Vec<T>]
{
    move |iter| {
        let mut ts: Vec<T> = vec![];
        while let Result::Ok(t) = parser(iter) {
            ts.push(t);
        }
        return Result::Ok(ts);
    }
}

pub const fn unwind<Iter: Clone, Err, T>(
    parser: parser![Iter, Err, T]
)
    -> parser![Iter, Err, T]
{
    move |iter| {
        let pre: Iter = iter.clone();
        let res = parser(iter);
        *iter = pre;
        res
    }
}