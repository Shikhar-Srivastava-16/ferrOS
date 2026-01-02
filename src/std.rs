#[allow(dead_code)]
pub trait Error {
    fn eprintf();
}

#[allow(dead_code)]
enum Result<T, E: Error> {
    Ok(T),
    Err(E),
}

#[allow(dead_code)]
enum Maybe<T> {
    Just(T),
    Nothing,
}

#[allow(dead_code)]
enum Either<Ty1, Ty2> {
    Left(Ty1),
    Right(Ty2),
}
