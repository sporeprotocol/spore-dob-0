pub mod dob_721;

mod type_casting {
    use alloc::{
        borrow::ToOwned,
        string::{String, ToString},
        vec::Vec,
    };
    use molecule::prelude::{Builder, Entity};

    use super::dob_721;

    impl From<dob_721::Number> for u64 {
        fn from(value: dob_721::Number) -> Self {
            Self::from_le_bytes(value.as_bytes().to_vec().try_into().unwrap())
        }
    }

    impl From<dob_721::String> for String {
        fn from(value: dob_721::String) -> Self {
            Self::from_utf8_lossy(&value.raw_data()).to_string()
        }
    }

    impl From<String> for dob_721::String {
        fn from(value: String) -> Self {
            Self::new_builder()
                .set(value.as_bytes().into_iter().map(|v| (*v).into()).collect())
                .build()
        }
    }

    impl From<Vec<String>> for dob_721::StringVec {
        fn from(value: Vec<String>) -> Self {
            Self::new_builder()
                .set(value.into_iter().map(Into::into).collect())
                .build()
        }
    }

    impl From<Vec<&str>> for dob_721::StringVec {
        fn from(value: Vec<&str>) -> Self {
            Self::new_builder()
                .set(value.into_iter().map(|v| v.to_owned().into()).collect())
                .build()
        }
    }

    impl From<u64> for dob_721::Number {
        fn from(value: u64) -> Self {
            Self::new_builder()
                .set(value.to_le_bytes().map(Into::into))
                .build()
        }
    }

    impl From<Vec<u64>> for dob_721::NumberVec {
        fn from(value: Vec<u64>) -> Self {
            Self::new_builder()
                .set(value.into_iter().map(Into::into).collect())
                .build()
        }
    }

    impl From<(Vec<u64>, u64)> for dob_721::FloatVec {
        fn from(value: (Vec<u64>, u64)) -> Self {
            Self::new_builder()
                .numerator_vec(value.0.into())
                .denominator(value.1.into())
                .build()
        }
    }

    impl From<(u64, u64)> for dob_721::NumberRange {
        fn from(value: (u64, u64)) -> Self {
            Self::new_builder()
                .set([value.0.into(), value.1.into()])
                .build()
        }
    }

    impl From<((u64, u64), u64)> for dob_721::FloatRange {
        fn from(value: ((u64, u64), u64)) -> Self {
            Self::new_builder()
                .numerator_range(value.0.into())
                .denominator(value.1.into())
                .build()
        }
    }
}

pub use type_casting::*;
