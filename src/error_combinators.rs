
pub fn fail<'a, Iter, Err, T>(
    msg: &'a impl Fn(&Iter) -> Err
)
    -> impl Fn(&mut Iter) -> Result<T, Err> + 'a
{
    |iter| {
        return Result::Err(msg(iter));
    }
}

pub fn fmap_err<'a, Iter, Err, Frr, T>(
    parser: &'a impl Fn(&mut Iter) -> Result<T, Err>,
    f: &'a impl Fn(Err) -> Frr
)
    -> impl Fn(&mut Iter) -> Result<T, Frr> + 'a
{
    move |iter| {
        parser(iter).map_err(f)
    }
}

pub fn unwind_on_err<'a, Iter: Clone, Err, T>(
    parser: &'a impl Fn(&mut Iter) -> Result<T, Err>
)
    -> impl Fn(&mut Iter) -> Result<T, Err> + 'a
{
    |iter| {
        let pre: Iter = iter.clone();
        match parser(iter) {
            Result::Ok(t) => Result::Ok(t),
            Result::Err(err) => {*iter = pre; Result::Err(err)}
        }
    }
}

pub fn recover_with<'a, Iter, Err, Frr, T>(
    parser: &'a impl Fn(&mut Iter) -> Result<T, Err>,
    recover: &'a impl Fn(&mut Iter) -> Result<(), Frr>
)
    -> impl Fn(&mut Iter) -> Result<Result<T, Err>, Frr> + 'a
{
    |iter| {
        match parser(iter) {
            Result::Ok(t) => Result::Ok(Result::Ok(t)),
            Result::Err(err) => match recover(iter) {
                Result::Ok(_) => Result::Ok(Result::Err(err)),
                Result::Err(frr) => Result::Err(frr)
            }
        }
    }
}

pub fn flatten_errors<'a, Iter, Err, T>(
    parser: &'a impl Fn(&mut Iter) -> Result<Result<T, Err>, Err>
)
    -> impl Fn(&mut Iter) -> Result<T, Err> + 'a
{
    |iter| {
        match parser(iter) {
            Result::Ok(Result::Ok(t)) => Result::Ok(t),
            Result::Ok(Result::Err(err)) => Result::Err(err),
            Result::Err(err) => Result::Err(err)
        }
    }
}