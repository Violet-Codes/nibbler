use super::parser;

pub fn lense<Iter, Jter, Err, T>(
    sect: impl Fn(&mut Jter) -> &mut Iter,
    parser: parser![Iter, Err, T]
)
    -> parser![Jter, Err, T]
{
    move |jter| parser(sect(jter))
}

pub fn run<Iter, Jter, Err, T, U>(
    build: impl Fn(&mut Jter) -> Iter,
    combine: impl Fn(&mut Iter, T) -> U,
    parser: parser![Iter, Err, T]
)
    -> parser![Jter, Err, U]
{
    move |jter| { let mut iter = build(jter); parser(&mut iter).map(|t| combine(&mut iter, t)) }
}

#[derive(Clone)]
pub struct CountIter<Iter> {
    iter: Iter,
    munched: usize
}

impl<Iter: Iterator> Iterator for CountIter<Iter> {
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Option::Some(token) => { self.munched += 1; Option::Some(token) },
            Option::None => Option::None
        }
    }

}

pub const fn count<Iter, Err>()
    -> parser![CountIter<Iter>, Err, usize]
{
    |iter| Result::Ok(iter.munched)
}

#[derive(Clone)]
pub struct StackIter<Iter, Symbol> {
    iter: Iter,
    stack: Vec<Symbol>
}

impl<Iter: Iterator, Symbol> Iterator for StackIter<Iter, Symbol> {
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub fn stack_action<Iter, Symbol: Clone + PartialEq, Err, const N: usize>(
    actions: [(bool, Symbol); N],
    pop_msg: impl Fn(&Iter, Symbol, Option<Symbol>) -> Err
)
    -> parser![StackIter<Iter, Symbol>, Err, ()]
{
    move |iter| {
        for action in actions.iter() {
            match action {
                (true, a) => iter.stack.push(a.clone()),
                (false, a) => match iter.stack.pop() {
                    Option::Some(b) => if a != &b { return Result::Err(pop_msg(& iter.iter, a.clone(), Option::Some(b))) },
                    Option::None => { return Result::Err(pop_msg(& iter.iter, a.clone(), Option::None)) }
                }
            };
        };
        Result::Ok(())
    }
}

pub fn stack_push<Iter, Symbol: Clone + PartialEq, Err>(
    a: Symbol
)
    -> parser![StackIter<Iter, Symbol>, Err, ()]
{
    move |iter| {
        iter.stack.push(a.clone());
        Result::Ok(())
    }
}

pub fn stack_pop<Iter, Symbol: Clone + PartialEq, Err>(
    a: Symbol,
    pop_msg: impl Fn(&Iter, Symbol, Option<Symbol>) -> Err
)
    -> parser![StackIter<Iter, Symbol>, Err, ()]
{
    move |iter| match iter.stack.pop() {
        Option::Some(b) => if a != b {
            Result::Err(pop_msg(& iter.iter, a.clone(), Option::Some(b)))
        } else {
            Result::Ok(())
        },
        Option::None => Result::Err(pop_msg(& iter.iter, a.clone(), Option::None))
    }
}

#[derive(Clone)]
pub struct CustomIter<Iter, State> {
    iter: Iter,
    state: State
}

impl<Iter: Iterator, State> Iterator for CustomIter<Iter, State> {
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub fn update_state<Iter, State, Err>(
    f: impl Fn(&mut State)
)
    -> parser![CustomIter<Iter, State>, Err, ()]
{
    move |iter| { f(&mut iter.state); Result::Ok(()) }
}

pub fn set_state<Iter, State, Err>(
    prod: impl Fn() -> State
)
    -> parser![CustomIter<Iter, State>, Err, ()]
{
    move |iter| { iter.state = prod(); Result::Ok(()) }
}

pub const fn get_state<Iter, State: Clone, Err>()
    -> parser![CustomIter<Iter, State>, Err, State]
{
    move |iter| Result::Ok(iter.state.clone())
}