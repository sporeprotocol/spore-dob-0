#![no_main]

use core::ffi::CStr;

extern crate alloc;
mod decoder;
mod schema;

#[no_mangle]
unsafe extern "C" fn main(argc: u64, argv: *const *const i8) {
    let mut args = Vec::new();
    for i in 1..argc {
        let argn = unsafe { CStr::from_ptr(argv.add(i as usize).read()) };
        args.push(argn.to_bytes());
    }
    match decoder::dobs_parse_parameters(args) {
        Ok(value) => match decoder::dobs_decode(value) {
            Ok(value) => {
                println!("result = {}", String::from_utf8_lossy(&value));
            }
            Err(err) => println!("error code = {}", err as u64),
        },
        Err(err) => println!("error code = {}", err as u64),
    }
}
