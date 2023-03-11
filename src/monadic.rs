use super::parser;

// pure
pub const fn pure<Iter, Err, T>(
    prod: impl Fn() -> T
)
    -> parser![Iter, Err, T]
{
    move |_iter| Result::Ok(prod())
}

// <$>
pub const fn fmap<Iter, Err, T, U>(
    f: impl Fn(T) -> U,
    parser: parser![Iter, Err, T]
)
    -> parser![Iter, Err, U]
{
    move |iter| parser(iter).map(& f)
}

pub const fn fmap2<Iter, Err, T, U, V>(
    f: impl Fn(T, U) -> V,
    t_parser: parser![Iter, Err, T],
    u_parser: parser![Iter, Err, U]
)
    -> parser![Iter, Err, V]
{
    move |iter| (|| {
        let t: T = match t_parser(iter) {
            Result::Ok(t) => t,
            Result::Err(err) => return Result::Err(err)
        };
        let u: U = match u_parser(iter) {
            Result::Ok(u) => u,
            Result::Err(err) => return Result::Err(err)
        };
        Result::Ok(f(t, u))
    })()
}

pub const fn fmap3<Iter, Err, T0, T1, T2, U>(
    f: impl Fn(T0, T1, T2) -> U,
    t0_parser: parser![Iter, Err, T0],
    t1_parser: parser![Iter, Err, T1],
    t2_parser: parser![Iter, Err, T2]
)
    -> parser![Iter, Err, U]
{
    move |iter| (|| {
        let t0: T0 = match t0_parser(iter) {
            Result::Ok(t0) => t0,
            Result::Err(err) => return Result::Err(err)
        };
        let t1: T1 = match t1_parser(iter) {
            Result::Ok(t1) => t1,
            Result::Err(err) => return Result::Err(err)
        };
        let t2: T2 = match t2_parser(iter) {
            Result::Ok(t2) => t2,
            Result::Err(err) => return Result::Err(err)
        };
        Result::Ok(f(t0, t1, t2))
    })()
}

pub const fn fmap4<Iter, Err, T0, T1, T2, T3, U>(
    f: impl Fn(T0, T1, T2, T3) -> U,
    t0_parser: parser![Iter, Err, T0],
    t1_parser: parser![Iter, Err, T1],
    t2_parser: parser![Iter, Err, T2],
    t3_parser: parser![Iter, Err, T3]
)
    -> parser![Iter, Err, U]
{
    move |iter| (|| {
        let t0: T0 = match t0_parser(iter) {
            Result::Ok(t0) => t0,
            Result::Err(err) => return Result::Err(err)
        };
        let t1: T1 = match t1_parser(iter) {
            Result::Ok(t1) => t1,
            Result::Err(err) => return Result::Err(err)
        };
        let t2: T2 = match t2_parser(iter) {
            Result::Ok(t2) => t2,
            Result::Err(err) => return Result::Err(err)
        };
        let t3: T3 = match t3_parser(iter) {
            Result::Ok(t3) => t3,
            Result::Err(err) => return Result::Err(err)
        };
        Result::Ok(f(t0, t1, t2, t3))
    })()
}

// <*>
pub const fn apply<Iter, Err, T, U, F: FnOnce(T) -> U>(
    f_parser: parser![Iter, Err, F],
    t_parser: parser![Iter, Err, T]
)
    -> parser![Iter, Err, U]
{
    move |iter| match f_parser(iter) {
        Result::Ok(f) => t_parser(iter).map(f),
        Result::Err(err) => Result::Err(err)
    }
}

pub const fn apply2<Iter, Err, T, U, V, F: FnOnce(T, U) -> V>(
    f_parser: parser![Iter, Err, F],
    t_parser: parser![Iter, Err, T],
    u_parser: parser![Iter, Err, U]
)
    -> parser![Iter, Err, V]
{
    move |iter| match f_parser(iter) {
        Result::Ok(f) => (|| {
            let t: T = match t_parser(iter) {
                Result::Ok(t) => t,
                Result::Err(err) => return Result::Err(err)
            };
            let u: U = match u_parser(iter) {
                Result::Ok(u) => u,
                Result::Err(err) => return Result::Err(err)
            };
            Result::Ok(f(t, u))
        })(),
        Result::Err(err) => Result::Err(err)
    }
}

pub const fn apply3<Iter, Err, T0, T1, T2, U, F: FnOnce(T0, T1, T2) -> U>(
    f_parser: parser![Iter, Err, F],
    t0_parser: parser![Iter, Err, T0],
    t1_parser: parser![Iter, Err, T1],
    t2_parser: parser![Iter, Err, T2]
)
    -> parser![Iter, Err, U]
{
    move |iter| match f_parser(iter) {
        Result::Ok(f) => (|| {
            let t0: T0 = match t0_parser(iter) {
                Result::Ok(t0) => t0,
                Result::Err(err) => return Result::Err(err)
            };
            let t1: T1 = match t1_parser(iter) {
                Result::Ok(t1) => t1,
                Result::Err(err) => return Result::Err(err)
            };
            let t2: T2 = match t2_parser(iter) {
                Result::Ok(t2) => t2,
                Result::Err(err) => return Result::Err(err)
            };
            Result::Ok(f(t0, t1, t2))
        })(),
        Result::Err(err) => Result::Err(err)
    }
}

pub const fn apply4<Iter, Err, T0, T1, T2, T3, U, F: FnOnce(T0, T1, T2, T3) -> U>(
    f_parser: parser![Iter, Err, F],
    t0_parser: parser![Iter, Err, T0],
    t1_parser: parser![Iter, Err, T1],
    t2_parser: parser![Iter, Err, T2],
    t3_parser: parser![Iter, Err, T3]
)
    -> parser![Iter, Err, U]
{
    move |iter| match f_parser(iter) {
        Result::Ok(f) => (|| {
            let t0: T0 = match t0_parser(iter) {
                Result::Ok(t0) => t0,
                Result::Err(err) => return Result::Err(err)
            };
            let t1: T1 = match t1_parser(iter) {
                Result::Ok(t1) => t1,
                Result::Err(err) => return Result::Err(err)
            };
            let t2: T2 = match t2_parser(iter) {
                Result::Ok(t2) => t2,
                Result::Err(err) => return Result::Err(err)
            };
            let t3: T3 = match t3_parser(iter) {
                Result::Ok(t3) => t3,
                Result::Err(err) => return Result::Err(err)
            };
            Result::Ok(f(t0, t1, t2, t3))
        })(),
        Result::Err(err) => Result::Err(err)
    }
}

// >>=
pub const fn bind<Iter, Err, T, U, UParser: FnOnce(&mut Iter) -> Result<U, Err>>(
    parser: parser![Iter, Err, T],
    f: impl Fn(T) -> UParser
)
    -> parser![Iter, Err, U]
{
    move |iter| match parser(iter) {
        Result::Ok(t) => f(t)(iter),
        Result::Err(err) => Result::Err(err)
    }
}

pub const fn bind2<Iter, Err, T, U, V, VParser: FnOnce(&mut Iter) -> Result<V, Err>>(
    t_parser: parser![Iter, Err, T],
    u_parser: parser![Iter, Err, U],
    f: impl Fn(T, U) -> VParser
)
    -> parser![Iter, Err, V]
{
    move |iter| (|| {
        let t: T = match t_parser(iter) {
            Result::Ok(t) => t,
            Result::Err(err) => return Result::Err(err)
        };
        let u: U = match u_parser(iter) {
            Result::Ok(u) => u,
            Result::Err(err) => return Result::Err(err)
        };
        f(t, u)(iter)
    })()
}

pub const fn bind3<Iter, Err, T0, T1, T2, U, UParser: FnOnce(&mut Iter) -> Result<U, Err>>(
    t0_parser: parser![Iter, Err, T0],
    t1_parser: parser![Iter, Err, T1],
    t2_parser: parser![Iter, Err, T2],
    f: impl Fn(T0, T1, T2) -> UParser
)
    -> parser![Iter, Err, U]
{
    move |iter| (|| {
        let t0: T0 = match t0_parser(iter) {
            Result::Ok(t0) => t0,
            Result::Err(err) => return Result::Err(err)
        };
        let t1: T1 = match t1_parser(iter) {
            Result::Ok(t1) => t1,
            Result::Err(err) => return Result::Err(err)
        };
        let t2: T2 = match t2_parser(iter) {
            Result::Ok(t2) => t2,
            Result::Err(err) => return Result::Err(err)
        };
        f(t0, t1, t2)(iter)
    })()
}

pub const fn bind4<Iter, Err, T0, T1, T2, T3, U, UParser: FnOnce(&mut Iter) -> Result<U, Err>>(
    t0_parser: parser![Iter, Err, T0],
    t1_parser: parser![Iter, Err, T1],
    t2_parser: parser![Iter, Err, T2],
    t3_parser: parser![Iter, Err, T3],
    f: impl Fn(T0, T1, T2, T3) -> UParser
)
    -> parser![Iter, Err, U]
{
    move |iter| (|| {
        let t0: T0 = match t0_parser(iter) {
            Result::Ok(t0) => t0,
            Result::Err(err) => return Result::Err(err)
        };
        let t1: T1 = match t1_parser(iter) {
            Result::Ok(t1) => t1,
            Result::Err(err) => return Result::Err(err)
        };
        let t2: T2 = match t2_parser(iter) {
            Result::Ok(t2) => t2,
            Result::Err(err) => return Result::Err(err)
        };
        let t3: T3 = match t3_parser(iter) {
            Result::Ok(t3) => t3,
            Result::Err(err) => return Result::Err(err)
        };
        f(t0, t1, t2, t3)(iter)
    })()
}

// <|>
pub const fn otherwise<Iter, Err, T>(
    parser0: parser![Iter, Vec<Err>, T],
    parser1: parser![Iter, Vec<Err>, T]
)
    -> parser![Iter, Vec<Err>, T]
{
    move |iter| match parser0(iter) {
        Result::Ok(t) => Result::Ok(t),
        Result::Err(mut err0) => match parser1(iter) {
            Result::Ok(t) => Result::Ok(t),
            Result::Err(mut err1) => Result::Err({ err0.append(&mut err1); err0 })
        }
    }
}
