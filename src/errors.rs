use super::parser;

pub const fn fail<Iter, Err, T>(
    msg: impl Fn(&Iter) -> Err
)
    -> parser![Iter, Err, T]
{
    move |iter| Result::Err(msg(iter))
}

pub const fn fmap_err<Iter, Err, Frr, T>(
    f: impl Fn(Err) -> Frr,
    parser: parser![Iter, Err, T]
)
    -> parser![Iter, Frr, T]
{
    move |iter| parser(iter).map_err(& f)
}

pub const fn try_parse<Iter: Clone, Err, T>(
    parser: parser![Iter, Err, T]
)
    -> parser![Iter, Err, T]
{
    move |iter| {
        let pre: Iter = iter.clone();
        match parser(iter) {
            Result::Ok(t) => Result::Ok(t),
            Result::Err(err) => {*iter = pre; Result::Err(err)}
        }
    }
}

pub const fn negate<Iter, Err, T>(
    parser: parser![Iter, Err, T]
)
    -> parser![Iter, T, Err]
{
    move |iter| match parser(iter) {
        Result::Ok(t) => Result::Err(t),
        Result::Err(err) => Result::Ok(err)
    }
}

pub const fn recover_with<Iter, Err, Frr, T>(
    parser: parser![Iter, Err, T],
    recover: parser![Iter, Frr, ()]
)
    -> parser![Iter, Frr, Result<T, Err>]
{
    move |iter| match parser(iter) {
        Result::Ok(t) => Result::Ok(Result::Ok(t)),
        Result::Err(err) => match recover(iter) {
            Result::Ok(_) => Result::Ok(Result::Err(err)),
            Result::Err(frr) => Result::Err(frr)
        }
    }
}

pub const fn flatten_errors<Iter, Err, T>(
    parser: parser![Iter, Err, Result<T, Err>]
)
    -> parser![Iter, Err, T]
{
    move |iter| match parser(iter) {
        Result::Ok(Result::Ok(t)) => Result::Ok(t),
        Result::Ok(Result::Err(err)) => Result::Err(err),
        Result::Err(err) => Result::Err(err)
    }
}

pub const fn wrap_err<Iter, Err, T>(
    parser: parser![Iter, Err, T]
)
    -> parser![Iter, Vec<Err>, T]
{
    move |iter| parser(iter).map_err(|err| vec![err])
}

pub const fn use_fst_err<Iter, Err, T>(
    parser: parser![Iter, Vec<Err>, T]
)
    -> parser![Iter, Err, T]
{
    move |iter| parser(iter).map_err(|mut errs| errs.remove(0))
}

pub const fn use_lst_err<Iter, Err, T>(
    parser: parser![Iter, Vec<Err>, T]
)
    -> parser![Iter, Err, T]
{
    move |iter| parser(iter).map_err(|mut errs| errs.remove(errs.len() - 1))
}