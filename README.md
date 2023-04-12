# nibbler
A simple and lightweight parser combinator library

## primer
The core type that nibbler operates with isn't a **type** at all, but is instead a trait, specifically:
```rs
impl Fn(&mut Iter) -> Result<T, Err>
```
This type is often written with the macro:
```rs
parser![Iter, Err, T]
```

This allows you to write simple inline parsers as closures that the library can operate on, these operations include:

### monad operations
From `nibbler::monadic`:

* `pure`:
```rs
// (pure) Turns a producer into a trivial parser
pub const fn pure<Iter, Err, T>(
    prod: impl Fn() -> T
)
    -> parser![Iter, Err, T];
```

* `fmap`: (varients include `fmap2`, `fmap3` and `fmap4`)
```rs
// (<$>) Applies a function to the resultant value
pub const fn fmap<Iter, Err, T, U>(
    f: impl Fn(T) -> U,
    parser: parser![Iter, Err, T]
)
    -> parser![Iter, Err, U];
```

* apply: (varients include `apply2`, `apply3` and `apply4`)
```rs
// (<*>) Applies the result of the 1st parser to the result from the 2nd parser
pub const fn apply<Iter, Err, T, U, F: FnOnce(T) -> U>(
    f_parser: parser![Iter, Err, F],
    t_parser: parser![Iter, Err, T]
)
    -> parser![Iter, Err, U];
```

* `bind`: (varients include `bind2`, `bind3` and `bind4`)
```rs
// (>>=) applies a function after parsing and then runs the result as its own parser
pub const fn bind<Iter, Err, T, U, UParser: FnOnce(&mut Iter) -> Result<U, Err>>(
    parser: parser![Iter, Err, T],
    f: impl Fn(T) -> UParser
)
    -> parser![Iter, Err, U];
```

* `alternative`
```rs
// (<|>) Runs the first parser and then, on error, runs the 2nd parser (does NOT rewind, use `error::try_parse` for that)
pub const fn otherwise<Iter, Err, T>(
    parser0: parser![Iter, Vec<Err>, T],
    parser1: parser![Iter, Vec<Err>, T]
)
    -> parser![Iter, Vec<Err>, T];
```
