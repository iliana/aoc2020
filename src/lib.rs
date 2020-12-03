pub use libc_print::*;

#[macro_export]
macro_rules! input {
    (@__prefix $prefix:expr) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/input/",
            $prefix,
            env!("CARGO_BIN_NAME"),
            ".txt",
        ))
    };

    () => {{
        cfg_if::cfg_if! {
            if #[cfg(test)] {
                input!(@__prefix "test_")
            } else {
                input!(@__prefix "")
            }
        }
    }};

    ($ty:ty) => {{
        use core::str::FromStr;
        input!().lines().map(|l| <$ty>::from_str(l).unwrap())
    }};
}
