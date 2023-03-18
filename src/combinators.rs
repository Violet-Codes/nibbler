use super::{ parser, errors::*, monadic::* };

pub const fn most_till<Iter, Err, T, U>(
    parser: parser![Iter, Err, T],
    end: parser![Iter, Err, U]
)
    -> parser![Iter, Vec<Err>, (Vec<T>, U)]
{
    fn most_till_inner<Iter_, Err_, T_, U_>(
        parser_: parser![Iter_, Err_, T_],
        end_: parser![Iter_, Err_, U_],
        iter: &mut Iter_
    )
        -> Result<(Vec<T_>, U_), Vec<Err_>>
    {
        otherwise(
            fmap2(
                |t, (mut ts, u): (Vec<T_>, U_)| { ts.insert(0, t); (ts, u) },
                wrap_err(& parser_),
                |iter| most_till_inner::<Iter_, Err_, T_, U_>(& parser_, & end_, iter)
            ),
            fmap(
                |u| (vec![], u),
                wrap_err(& end_)
            )
        )(iter)
    }
    move |iter| most_till_inner(& parser, & end, iter)
}

pub const fn least_till<Iter, Err, T, U>(
    parser: parser![Iter, Err, T],
    end: parser![Iter, Err, U]
)
    -> parser![Iter, Vec<Err>, (Vec<T>, U)]
{
    fn least_till_inner<Iter_, Err_, T_, U_>(
        parser_: parser![Iter_, Err_, T_],
        end_: parser![Iter_, Err_, U_],
        iter: &mut Iter_
    )
        -> Result<(Vec<T_>, U_), Vec<Err_>>
    {
        otherwise(
            fmap(
                |u| (vec![], u),
                wrap_err(& end_)
            ),
            fmap2(
                |t, (mut ts, u): (Vec<T_>, U_)| { ts.insert(0, t); (ts, u) },
                wrap_err(& parser_),
                |iter| least_till_inner::<Iter_, Err_, T_, U_>(& parser_, & end_, iter)
            )
        )(iter)
    }
    move |iter| least_till_inner(& parser, & end, iter)
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