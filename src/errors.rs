pub fn fail<Iter, Err, T>(
    msg: impl Fn(&Iter) -> Err
)
    -> impl Fn(&mut Iter) -> Result<T, Err>
{
    move |iter| Result::Err(msg(iter))
}

pub fn fmap_err<Iter, Err, Frr, T>(
    f: impl Fn(Err) -> Frr,
    parser: impl Fn(&mut Iter) -> Result<T, Err>
)
    -> impl Fn(&mut Iter) -> Result<T, Frr>
{
    move |iter| parser(iter).map_err(& f)
}

pub fn try_parse<Iter: Clone, Err, T>(
    parser: impl Fn(&mut Iter) -> Result<T, Err>
)
    -> impl Fn(&mut Iter) -> Result<T, Err>
{
    move |iter| {
        let pre: Iter = iter.clone();
        match parser(iter) {
            Result::Ok(t) => Result::Ok(t),
            Result::Err(err) => {*iter = pre; Result::Err(err)}
        }
    }
}

pub fn negate<Iter, Err, T>(
    parser: impl Fn(&mut Iter) -> Result<T, Err>
)
    -> impl Fn(&mut Iter) -> Result<Err, T>
{
    move |iter| match parser(iter) {
        Result::Ok(t) => Result::Err(t),
        Result::Err(err) => Result::Ok(err)
    }
}

pub fn recover_with<Iter, Err, Frr, T>(
    parser: impl Fn(&mut Iter) -> Result<T, Err>,
    recover: impl Fn(&mut Iter) -> Result<(), Frr>
)
    -> impl Fn(&mut Iter) -> Result<Result<T, Err>, Frr>
{
    move |iter| match parser(iter) {
        Result::Ok(t) => Result::Ok(Result::Ok(t)),
        Result::Err(err) => match recover(iter) {
            Result::Ok(_) => Result::Ok(Result::Err(err)),
            Result::Err(frr) => Result::Err(frr)
        }
    }
}

pub fn flatten_errors<Iter, Err, T>(
    parser: impl Fn(&mut Iter) -> Result<Result<T, Err>, Err>
)
    -> impl Fn(&mut Iter) -> Result<T, Err>
{
    move |iter| match parser(iter) {
        Result::Ok(Result::Ok(t)) => Result::Ok(t),
        Result::Ok(Result::Err(err)) => Result::Err(err),
        Result::Err(err) => Result::Err(err)
    }
}

pub fn wrap_err<Iter, Err, T>(
    parser: impl Fn(&mut Iter) -> Result<T, Err>
)
    -> impl Fn(&mut Iter) -> Result<T, Vec<Err>>
{
    move |iter| parser(iter).map_err(|err| vec![err])
}

pub fn use_fst_err<Iter, Err, T>(
    parser: impl Fn(&mut Iter) -> Result<T, Vec<Err>>
)
    -> impl Fn(&mut Iter) -> Result<T, Err>
{
    move |iter| parser(iter).map_err(|mut errs| errs.remove(0))
}

pub fn use_lst_err<Iter, Err, T>(
    parser: impl Fn(&mut Iter) -> Result<T, Vec<Err>>
)
    -> impl Fn(&mut Iter) -> Result<T, Err>
{
    move |iter| parser(iter).map_err(|mut errs| errs.remove(errs.len() - 1))
}