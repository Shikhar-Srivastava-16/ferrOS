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

// #[macro_export]
// macro_rules! print {
//     ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
// }
//
// #[macro_export]
// macro_rules! println {
//     () => ($crate::print!("\n"));
//     ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
// }
