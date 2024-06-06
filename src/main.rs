#![no_main]
#![no_std]

extern crate alloc;
use core::ffi::CStr;

use alloc::{format, vec::Vec};
use spore_dob_0::decoder::{dobs_decode, dobs_parse_parameters};

const HEAPS_SIZE: usize = 1024 * 64;

static mut HEAPS: [u8; HEAPS_SIZE] = [0; HEAPS_SIZE];
#[global_allocator]
static ALLOC: linked_list_allocator::LockedHeap = linked_list_allocator::LockedHeap::empty();

#[panic_handler]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    // If the main thread panics it will terminate all your threads and end your program with code 101.
    // See: https://github.com/rust-lang/rust/blob/master/library/core/src/macros/panic.md
    syscall_write(format!("{panic_info:?}").as_ptr());
    syscall_exit(101)
}

fn syscall(mut a0: u64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64, a7: u64) -> u64 {
    unsafe {
        core::arch::asm!(
          "ecall",
          inout("a0") a0,
          in("a1") a1,
          in("a2") a2,
          in("a3") a3,
          in("a4") a4,
          in("a5") a5,
          in("a6") a6,
          in("a7") a7
        )
    }
    a0
}

fn syscall_exit(code: u64) -> ! {
    syscall(code, 0, 0, 0, 0, 0, 0, 93);
    loop {}
}

pub fn syscall_write(buf: *const u8) -> u64 {
    syscall(buf as u64, 0, 0, 0, 0, 0, 0, 2177)
}

#[no_mangle]
pub unsafe extern "C" fn _start() {
    core::arch::asm!(
        "lw a0,0(sp)", // Argc.
        "add a1,sp,8", // Argv.
        "li a2,0",     // Envp.
        "call main",
        "li a7, 93",
        "ecall",
    );
}

#[no_mangle]
unsafe extern "C" fn main(argc: u64, argv: *const *const i8) -> u64 {
    unsafe {
        ALLOC.lock().init(HEAPS.as_mut_ptr(), HEAPS_SIZE);
    }

    let mut args = Vec::new();
    for i in 0..argc {
        let argn = unsafe { CStr::from_ptr(argv.add(i as usize).read()) };
        args.push(argn.to_bytes());
    }
    let dob_params = match dobs_parse_parameters(args) {
        Ok(value) => value,
        Err(err) => return err as u64,
    };
    match dobs_decode(dob_params) {
        Ok(mut bytes) => {
            bytes.push(0);
            syscall_write(bytes.as_ptr() as *const u8);
            return 0;
        }
        Err(error) => return error as u64,
    }
}
