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

## monad operations
These are used to deal with the "side effects" of the parser type, located in `nibbler::monadic` are:

* `pure`:
```rs
/// (pure) Turns a producer into a trivial parser
pub const fn pure<Iter, Err, T>(
    prod: impl Fn() -> T
)
    -> parser![Iter, Err, T];
```

* `fmap`: (varients include `fmap2`, `fmap3` and `fmap4`)
```rs
/// (<$>) Applies a function to the resultant value
pub const fn fmap<Iter, Err, T, U>(
    f: impl Fn(T) -> U,
    parser: parser![Iter, Err, T]
)
    -> parser![Iter, Err, U];
```

* apply: (varients include `apply2`, `apply3` and `apply4`)
```rs
/// (<*>) Applies the result of the 1st parser to the result from the 2nd parser
pub const fn apply<Iter, Err, T, U, F: FnOnce(T) -> U>(
    f_parser: parser![Iter, Err, F],
    t_parser: parser![Iter, Err, T]
)
    -> parser![Iter, Err, U];
```

* `bind`: (varients include `bind2`, `bind3` and `bind4`)
```rs
/// (>>=) Applies a function after parsing and then runs the result as its own parser
pub const fn bind<Iter, Err, T, U, UParser: FnOnce(&mut Iter) -> Result<U, Err>>(
    parser: parser![Iter, Err, T],
    f: impl Fn(T) -> UParser
)
    -> parser![Iter, Err, U];
```

* `otherwise`
```rs
/// (<|>) Runs the first parser and then, on error, runs the 2nd parser
/// (DOES 👏 NOT 👏 REWIND 👏, use `error::try_parse` for that)
pub const fn otherwise<Iter, Err, T>(
    parser0: parser![Iter, Vec<Err>, T],
    parser1: parser![Iter, Vec<Err>, T]
)
    -> parser![Iter, Vec<Err>, T];
```

## error handling
These are used to deal with the "side effects" of the parser type, located in `nibbler::errors` are:

* `fail`:
```rs
/// Raises an error using the state
pub const fn fail<Iter, Err, T>(
    msg: impl Fn(&Iter) -> Err
)
    -> parser![Iter, Err, T];
```

 * `fmap_err_with_state`: (simple case is: `fmap_err`)
```rs
/// Modifies the error on error path using the state (BEFORE 👏 PARSING 👏) to generate an `FnOnce` action
pub const fn fmap_err_with_state<Iter, Err, Frr, T, G: FnOnce(Err) -> Frr>(
    f: impl Fn(&Iter) -> G,
    parser: parser![Iter, Err, T]
)
    -> parser![Iter, Frr, T];
```

* `try_parse`:
```rs
/// Copies the state before parsing and sets the state back on error path
pub const fn try_parse<Iter: Clone, Err, T>(
    parser: parser![Iter, Err, T]
)
    -> parser![Iter, Err, T];
```

## error recovery
These are used to break from the error path and potentially re-enter with more information, located in `nibbler::errors` are:

* `recover_with`:
```rs
/// Recovers from the error path using the recovery parser and returns the error with the `Err` pattern for result
pub const fn recover_with<Iter, Err, Frr, T>(
    parser: parser![Iter, Err, T],
    recover: parser![Iter, Frr, ()]
)
    -> parser![Iter, Frr, Result<T, Err>];
```

* `flatten_errors`:
```rs
/// The opposite of `recover_with`; starts the error if the result type pattern `Err`
pub const fn flatten_errors<Iter, Err, T>(
    parser: parser![Iter, Err, Result<T, Err>]
)
    -> parser![Iter, Err, T];
```
