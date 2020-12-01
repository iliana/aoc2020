pub use libc_print::*;

#[macro_export]
macro_rules! input {
    (@inner $ty:ty, $prefix:expr) => {{
        use core::str::FromStr;
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/input/",
            $prefix,
            env!("CARGO_BIN_NAME"),
            ".txt",
        ))
        .lines()
        .map(|l| <$ty>::from_str(l).unwrap())
    }};

    ($ty:ty) => {
        input!(@inner $ty, "")
    };
}

#[macro_export]
macro_rules! test_input {
    ($ty:ty) => {
        input!(@inner $ty, "test_")
    };
}
