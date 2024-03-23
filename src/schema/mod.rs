pub mod dob_721;

mod type_casting {
    use alloc::string::{String, ToString};
    use molecule::prelude::Entity;

    use super::dob_721;

    impl From<dob_721::Number> for u64 {
        fn from(value: dob_721::Number) -> Self {
            Self::from_le_bytes(value.as_bytes().to_vec().try_into().unwrap())
        }
    }

    impl From<dob_721::String> for String {
        fn from(value: dob_721::String) -> Self {
            Self::from_utf8_lossy(&value.as_bytes()).to_string()
        }
    }
}

pub use type_casting::*;
