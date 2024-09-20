#[macro_export]
macro_rules! unwrap_or_err {
    ( $e:expr, $t:tt ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return Err($t),
        }
    }
}