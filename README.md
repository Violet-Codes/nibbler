# nibbler
A simple and lightweight parser combinator library

## primer
The core type that nibbler operates with isn't a **type** at all, but is instead a trait, specifically:
```rs
impl Fn(&mut Iter) -> Result<T, Err>
```
