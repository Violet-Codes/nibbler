pub fn lense<Iter, Jter, Err, T>(
    sect: impl Fn(&mut Jter) -> &mut Iter,
    parser: impl Fn(&mut Iter) -> Result<T, Err>
)
    -> impl Fn(&mut Jter) -> Result<T, Err>
{
    move |jter| parser(sect(jter))
}

pub fn run<Iter, Jter, Err, T>(
    build: impl Fn(&mut Jter) -> Iter,
    parser: impl Fn(&mut Iter) -> Result<T, Err>
)
    -> impl Fn(&mut Jter) -> Result<T, Err>
{
    move |jter| { let mut iter = build(jter); parser(&mut iter) }
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
    -> impl Fn(&mut CountIter<Iter>) -> Result<usize, Err>
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
    -> impl Fn(&mut StackIter<Iter, Symbol>) -> Result<(), Err>
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
    -> impl Fn(&mut StackIter<Iter, Symbol>) -> Result<(), Err>
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
    -> impl Fn(&mut StackIter<Iter, Symbol>) -> Result<(), Err>
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
    -> impl Fn(&mut CustomIter<Iter, State>) -> Result<(), Err>
{
    move |iter| { f(&mut iter.state); Result::Ok(()) }
}

pub fn set_state<Iter, State: Clone, Err>(
    s: State
)
    -> impl Fn(&mut CustomIter<Iter, State>) -> Result<(), Err>
{
    move |iter| { iter.state = s.clone(); Result::Ok(()) }
}

pub const fn get_state<Iter, State: Clone, Err>()
    -> impl Fn(&mut CustomIter<Iter, State>) -> Result<State, Err>
{
    move |iter| Result::Ok(iter.state.clone())
}

pub fn run_state<Iter, State, Err, T>(
    build: impl Fn(&Iter) -> State,
    parser: impl Fn(&mut CustomIter<&mut Iter, State>) -> Result<T, Err>
)
    -> impl Fn(&mut Iter) -> Result<(State, T), Err>
{
    move |iter| {
        let state = build(iter);
        let mut jter = CustomIter{ iter: iter, state: state };
        parser(&mut jter).map(|t| (jter.state, t))
    }
}