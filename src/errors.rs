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

pub const fn fmap_err_with_state<Iter, Err, Frr, T, G: FnOnce(Err) -> Frr>(
    f: impl Fn(&Iter) -> G,
    parser: parser![Iter, Err, T]
)
    -> parser![Iter, Frr, T]
{
    move |iter| {
        let g = f(iter);
        parser(iter).map_err(g)
    }
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

#[derive(Debug, Clone)]
pub enum ParseError<Info>{
    Silent,
    Message(String, Info),
    Contextual(String, Info, Box<Self>),
    ErrBundle(Vec<Self>),
    ErrChoice(Vec<Self>)
}

pub fn show_error<Info>(
    padding: String,
    show_info: & impl Fn(Info) -> String,
    parse_err: ParseError<Info>
)
    -> String
{
    match parse_err {
        ParseError::Silent =>
            format!("{padding}a silent error occured..."),
        ParseError::Message(name, info) =>
            format!("{padding}expected {} {}...", name, show_info(info)),
        ParseError::Contextual(name, info, err) =>
            format!("{}\n{padding}...whilst parsing {} {}...", show_error(padding.clone(), show_info, *err), name, (show_info)(info)),
        ParseError::ErrBundle(errs) =>
            format!(
                "{}{padding}...grouped here",
                errs.into_iter().map(|err|
                    format!("{}\n{padding}|- in error bundle...\n", show_error(format!("{padding}| "), show_info, err))
                ).collect::<String>()
            ),
        ParseError::ErrChoice(errs) =>
            format!(
                "{}{padding}...branching here",
                errs.into_iter().map(|err|
                    format!("{}\n{padding}|- in choice...\n", show_error(format!("{padding}| "), show_info, err))
                ).collect::<String>()
            )
    }
}

pub fn truncate_parse_err<Info>(
    err: ParseError<Info>
)
    -> ParseError<Info>
{
    match err {
        ParseError::Silent => ParseError::Silent,
        ParseError::Message(name, info) => ParseError::Message(name, info),
        ParseError::Contextual(name, info, _ctx) => ParseError::Message(name, info),
        ParseError::ErrBundle(errs) => ParseError::ErrBundle(errs.into_iter().map(truncate_parse_err).collect()),
        ParseError::ErrChoice(errs) => ParseError::ErrChoice(errs.into_iter().map(truncate_parse_err).collect()),
    }
}

pub const fn bundle<Iter, Info, T>(
    parser: parser![Iter, Vec<ParseError<Info>>, T]
)
    -> parser![Iter, ParseError<Info>, T]
{
    move |iter| fmap_err(|errs| ParseError::ErrBundle(errs), & parser)(iter)
}

pub fn label<Iter, Info, T>(
    name: String,
    info_getter: impl Fn(&Iter) -> Info,
    parser: parser![Iter, ParseError<Info>, T]
)
    -> parser![Iter, ParseError<Info>, T]
{
    fmap_err_with_state(
        move |iter| {
            let info: Info = info_getter(iter);
            let name_clone = name.clone();
            |err| match err {
                ParseError::Silent => ParseError::Message(name_clone, info),
                err_ => ParseError::Contextual(name_clone, info, Box::new(err_))
            }
        },
        parser
    )
}

pub const fn display_full_choice<Iter, Info, T>(
    parser: parser![Iter, Vec<ParseError<Info>>, T]
)
    -> parser![Iter, ParseError<Info>, T]
{
    move |iter| fmap_err(
        |mut errs| {
            errs = errs
                .into_iter()
                .filter_map(
                    |err| match err {
                        ParseError::Silent => None,
                        _err => Some(_err)
                    }
                )
                .collect();
            if errs.len() != 0 {
                ParseError::ErrChoice(errs)
            } else {
                ParseError::Silent
            }
        },
        & parser
    )(iter)
}

pub const fn display_fst_choice<Iter, Info, T>(
    parser: parser![Iter, Vec<ParseError<Info>>, T]
)
    -> parser![Iter, ParseError<Info>, T]
{
    move |iter| fmap_err(
        |mut errs| {
            errs = errs
                .into_iter()
                .filter_map(
                    |err| match err {
                        ParseError::Silent => None,
                        _err => Some(_err)
                    }
                )
                .collect();
            errs.reverse();
            if let Some(fst) = errs.pop() {
                errs = errs.into_iter().map(truncate_parse_err).collect();
                errs.push(fst);
                errs.reverse();
                ParseError::ErrChoice(errs)
            } else {
                ParseError::Silent
            }
        },
        & parser
    )(iter)
}

pub const fn display_lst_choice<Iter, Info, T>(
    parser: parser![Iter, Vec<ParseError<Info>>, T]
)
    -> parser![Iter, ParseError<Info>, T]
{
    move |iter| fmap_err(
        |mut errs| {
            errs = errs
                .into_iter()
                .filter_map(
                    |err| match err {
                        ParseError::Silent => None,
                        _err => Some(_err)
                    }
                )
                .collect();
            if let Some(fst) = errs.pop() {
                errs = errs.into_iter().map(truncate_parse_err).collect();
                errs.push(fst);
                ParseError::ErrChoice(errs)
            } else {
                ParseError::Silent
            }
        },
        & parser
    )(iter)
}