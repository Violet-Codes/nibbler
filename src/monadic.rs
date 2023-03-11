// Monadic parser combinators

// pure
pub fn pure<Iter, Err, T>(
    prod: impl Fn() -> T
)
    -> impl Fn(&mut Iter) -> Result<T, Err>
{
    move |_iter| Result::Ok(prod())
}

// <$>
pub fn fmap<Iter, Err, T, U>(
    f: impl Fn(T) -> U,
    parser: impl Fn(&mut Iter) -> Result<T, Err>
)
    -> impl Fn(&mut Iter) -> Result<U, Err>
{
    move |iter| parser(iter).map(& f)
}

pub fn fmap2<Iter, Err, T, U, V>(
    f: impl Fn(T, U) -> V,
    t_parser: impl Fn(&mut Iter) -> Result<T, Err>,
    u_parser: impl Fn(&mut Iter) -> Result<U, Err>
)
    -> impl Fn(&mut Iter) -> Result<V, Err>
{
    move |iter| 'parse_args: loop {
        let t: T = match t_parser(iter) {
            Result::Ok(t) => t,
            Result::Err(err) => break 'parse_args Result::Err(err)
        };
        let u: U = match u_parser(iter) {
            Result::Ok(u) => u,
            Result::Err(err) => break 'parse_args Result::Err(err)
        };
        break Result::Ok(f(t, u));
    }
}

pub fn fmap3<Iter, Err, T0, T1, T2, U>(
    f: impl Fn(T0, T1, T2) -> U,
    t0_parser: impl Fn(&mut Iter) -> Result<T0, Err>,
    t1_parser: impl Fn(&mut Iter) -> Result<T1, Err>,
    t2_parser: impl Fn(&mut Iter) -> Result<T2, Err>
)
    -> impl Fn(&mut Iter) -> Result<U, Err>
{
    move |iter| 'parse_args: loop {
        let t0: T0 = match t0_parser(iter) {
            Result::Ok(t0) => t0,
            Result::Err(err) => break 'parse_args Result::Err(err)
        };
        let t1: T1 = match t1_parser(iter) {
            Result::Ok(t1) => t1,
            Result::Err(err) => break 'parse_args Result::Err(err)
        };
        let t2: T2 = match t2_parser(iter) {
            Result::Ok(t2) => t2,
            Result::Err(err) => break 'parse_args Result::Err(err)
        };
        break Result::Ok(f(t0, t1, t2));
    }
}

pub fn fmap4<Iter, Err, T0, T1, T2, T3, U>(
    f: impl Fn(T0, T1, T2, T3) -> U,
    t0_parser: impl Fn(&mut Iter) -> Result<T0, Err>,
    t1_parser: impl Fn(&mut Iter) -> Result<T1, Err>,
    t2_parser: impl Fn(&mut Iter) -> Result<T2, Err>,
    t3_parser: impl Fn(&mut Iter) -> Result<T3, Err>
)
    -> impl Fn(&mut Iter) -> Result<U, Err>
{
    move |iter| 'parse_args: loop {
        let t0: T0 = match t0_parser(iter) {
            Result::Ok(t0) => t0,
            Result::Err(err) => break 'parse_args Result::Err(err)
        };
        let t1: T1 = match t1_parser(iter) {
            Result::Ok(t1) => t1,
            Result::Err(err) => break 'parse_args Result::Err(err)
        };
        let t2: T2 = match t2_parser(iter) {
            Result::Ok(t2) => t2,
            Result::Err(err) => break 'parse_args Result::Err(err)
        };
        let t3: T3 = match t3_parser(iter) {
            Result::Ok(t3) => t3,
            Result::Err(err) => break 'parse_args Result::Err(err)
        };
        break Result::Ok(f(t0, t1, t2, t3));
    }
}

// <*>
pub fn apply<Iter, Err, T, U, F: FnOnce(T) -> U>(
    f_parser: impl Fn(&mut Iter) -> Result<F, Err>,
    t_parser: impl Fn(&mut Iter) -> Result<T, Err>
)
    -> impl Fn(&mut Iter) -> Result<U, Err>
{
    move |iter| match f_parser(iter) {
        Result::Ok(f) => t_parser(iter).map(f),
        Result::Err(err) => Result::Err(err)
    }
}

pub fn apply2<Iter, Err, T, U, V, F: FnOnce(T, U) -> V>(
    f_parser: impl Fn(&mut Iter) -> Result<F, Err>,
    t_parser: impl Fn(&mut Iter) -> Result<T, Err>,
    u_parser: impl Fn(&mut Iter) -> Result<U, Err>
)
    -> impl Fn(&mut Iter) -> Result<V, Err>
{
    move |iter| match f_parser(iter) {
        Result::Ok(f) => 'parse_args: loop {
            let t: T = match t_parser(iter) {
                Result::Ok(t) => t,
                Result::Err(err) => break 'parse_args Result::Err(err)
            };
            let u: U = match u_parser(iter) {
                Result::Ok(u) => u,
                Result::Err(err) => break 'parse_args Result::Err(err)
            };
            break Result::Ok(f(t, u));
        },
        Result::Err(err) => Result::Err(err)
    }
}

pub fn apply3<Iter, Err, T0, T1, T2, U, F: FnOnce(T0, T1, T2) -> U>(
    f_parser: impl Fn(&mut Iter) -> Result<F, Err>,
    t0_parser: impl Fn(&mut Iter) -> Result<T0, Err>,
    t1_parser: impl Fn(&mut Iter) -> Result<T1, Err>,
    t2_parser: impl Fn(&mut Iter) -> Result<T2, Err>
)
    -> impl Fn(&mut Iter) -> Result<U, Err>
{
    move |iter| match f_parser(iter) {
        Result::Ok(f) => 'parse_args: loop {
            let t0: T0 = match t0_parser(iter) {
                Result::Ok(t0) => t0,
                Result::Err(err) => break 'parse_args Result::Err(err)
            };
            let t1: T1 = match t1_parser(iter) {
                Result::Ok(t1) => t1,
                Result::Err(err) => break 'parse_args Result::Err(err)
            };
            let t2: T2 = match t2_parser(iter) {
                Result::Ok(t2) => t2,
                Result::Err(err) => break 'parse_args Result::Err(err)
            };
            break Result::Ok(f(t0, t1, t2));
        },
        Result::Err(err) => Result::Err(err)
    }
}

pub fn apply4<Iter, Err, T0, T1, T2, T3, U, F: FnOnce(T0, T1, T2, T3) -> U>(
    f_parser: impl Fn(&mut Iter) -> Result<F, Err>,
    t0_parser: impl Fn(&mut Iter) -> Result<T0, Err>,
    t1_parser: impl Fn(&mut Iter) -> Result<T1, Err>,
    t2_parser: impl Fn(&mut Iter) -> Result<T2, Err>,
    t3_parser: impl Fn(&mut Iter) -> Result<T3, Err>
)
    -> impl Fn(&mut Iter) -> Result<U, Err>
{
    move |iter| match f_parser(iter) {
        Result::Ok(f) => 'parse_args: loop {
            let t0: T0 = match t0_parser(iter) {
                Result::Ok(t0) => t0,
                Result::Err(err) => break 'parse_args Result::Err(err)
            };
            let t1: T1 = match t1_parser(iter) {
                Result::Ok(t1) => t1,
                Result::Err(err) => break 'parse_args Result::Err(err)
            };
            let t2: T2 = match t2_parser(iter) {
                Result::Ok(t2) => t2,
                Result::Err(err) => break 'parse_args Result::Err(err)
            };
            let t3: T3 = match t3_parser(iter) {
                Result::Ok(t3) => t3,
                Result::Err(err) => break 'parse_args Result::Err(err)
            };
            break Result::Ok(f(t0, t1, t2, t3));
        },
        Result::Err(err) => Result::Err(err)
    }
}

// >>=
pub fn bind<Iter, Err, T, U, UParser: FnOnce(&mut Iter) -> Result<U, Err>>(
    parser: impl Fn(&mut Iter) -> Result<T, Err>,
    f: impl Fn(T) -> UParser
)
    -> impl Fn(&mut Iter) -> Result<U, Err>
{
    move |iter| match parser(iter) {
        Result::Ok(t) => f(t)(iter),
        Result::Err(err) => Result::Err(err)
    }
}

pub fn bind2<Iter, Err, T, U, V, VParser: FnOnce(&mut Iter) -> Result<V, Err>>(
    t_parser: impl Fn(&mut Iter) -> Result<T, Err>,
    u_parser: impl Fn(&mut Iter) -> Result<U, Err>,
    f: impl Fn(T, U) -> VParser
)
    -> impl Fn(&mut Iter) -> Result<V, Err>
{
    move |iter| 'parse_args: loop {
        let t: T = match t_parser(iter) {
            Result::Ok(t) => t,
            Result::Err(err) => break 'parse_args Result::Err(err)
        };
        let u: U = match u_parser(iter) {
            Result::Ok(u) => u,
            Result::Err(err) => break 'parse_args Result::Err(err)
        };
        break f(t, u)(iter);
    }
}

pub fn bind3<Iter, Err, T0, T1, T2, U, UParser: FnOnce(&mut Iter) -> Result<U, Err>>(
    t0_parser: impl Fn(&mut Iter) -> Result<T0, Err>,
    t1_parser: impl Fn(&mut Iter) -> Result<T1, Err>,
    t2_parser: impl Fn(&mut Iter) -> Result<T2, Err>,
    f: impl Fn(T0, T1, T2) -> UParser
)
    -> impl Fn(&mut Iter) -> Result<U, Err>
{
    move |iter| 'parse_args: loop {
        let t0: T0 = match t0_parser(iter) {
            Result::Ok(t0) => t0,
            Result::Err(err) => break 'parse_args Result::Err(err)
        };
        let t1: T1 = match t1_parser(iter) {
            Result::Ok(t1) => t1,
            Result::Err(err) => break 'parse_args Result::Err(err)
        };
        let t2: T2 = match t2_parser(iter) {
            Result::Ok(t2) => t2,
            Result::Err(err) => break 'parse_args Result::Err(err)
        };
        break f(t0, t1, t2)(iter);
    }
}

pub fn bind4<Iter, Err, T0, T1, T2, T3, U, UParser: FnOnce(&mut Iter) -> Result<U, Err>>(
    t0_parser: impl Fn(&mut Iter) -> Result<T0, Err>,
    t1_parser: impl Fn(&mut Iter) -> Result<T1, Err>,
    t2_parser: impl Fn(&mut Iter) -> Result<T2, Err>,
    t3_parser: impl Fn(&mut Iter) -> Result<T3, Err>,
    f: impl Fn(T0, T1, T2, T3) -> UParser
)
    -> impl Fn(&mut Iter) -> Result<U, Err>
{
    move |iter| 'parse_args: loop {
        let t0: T0 = match t0_parser(iter) {
            Result::Ok(t0) => t0,
            Result::Err(err) => break 'parse_args Result::Err(err)
        };
        let t1: T1 = match t1_parser(iter) {
            Result::Ok(t1) => t1,
            Result::Err(err) => break 'parse_args Result::Err(err)
        };
        let t2: T2 = match t2_parser(iter) {
            Result::Ok(t2) => t2,
            Result::Err(err) => break 'parse_args Result::Err(err)
        };
        let t3: T3 = match t3_parser(iter) {
            Result::Ok(t3) => t3,
            Result::Err(err) => break 'parse_args Result::Err(err)
        };
        break f(t0, t1, t2, t3)(iter);
    }
}

// Alternative parser combinators

// <|>
// Vec generalises any associative error combination
pub fn otherwise<Iter, Err, T>(
    parser0: impl Fn(&mut Iter) -> Result<T, Vec<Err>>,
    parser1: impl Fn(&mut Iter) -> Result<T, Vec<Err>>
)
    -> impl Fn(&mut Iter) -> Result<T, Vec<Err>>
{
    move |iter| match parser0(iter) {
        Result::Ok(t) => Result::Ok(t),
        Result::Err(mut err0) => match parser1(iter) {
            Result::Ok(t) => Result::Ok(t),
            Result::Err(mut err1) => Result::Err({ err0.append(&mut err1); err0 })
        }
    }
}