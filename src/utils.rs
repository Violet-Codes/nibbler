fn result_append<Err, T>(
    resvec: &mut Result<Vec<T>, Vec<Err>>,
    resval: Result<T, Err>
)
    -> ()
{
    match resvec {
        Result::Ok(ts) => match resval {
            Result::Ok(t) => ts.push(t),
            Result::Err(err) => *resvec = Result::Err(vec![err])
        },
        Result::Err(errs) => match resval {
            Result::Err(err) => errs.push(err),
            _ => {}
        }
    };
}