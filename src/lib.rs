#![no_std]

extern crate alloc;
pub mod decoder;
pub mod schema;

#[cfg(test)]
mod test {
    use alloc::string::String;

    use crate::schema::dob_721;

    #[test]
    fn test_generate_decode() {
        unimplemented!()
    }
}
