pub use libc_print::*;

#[macro_export]
macro_rules! input_name {
    ($prefix:expr) => {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/input/",
            $prefix,
            env!("CARGO_BIN_NAME"),
            ".txt",
        )
    };

    () => {
        input_name!("")
    };
}

#[macro_export]
macro_rules! test_input_name {
    () => {
        input_name!("test_")
    };
}

#[macro_export]
macro_rules! input {
    (@inner $ty:ty, $prefix:expr) => {{
        use core::str::FromStr;
        include_str!(input_name!($prefix))
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
