#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;
use risc_v::*;

//const DEVICE_BASE_ADDRESS: usize = 0xff200020;
//const DEVICE_BASE_ADDRESS: usize = 0x80000000;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    //let device_ptr = DEVICE_BASE_ADDRESS as *mut u8;
    //let counter = 0;
    //unsafe {
    //    *device_ptr = 10_usize;
    //}

    println!("counter");
    loop {
        shutdown();
    }
}

//#[no_mangle]
//pub unsafe extern "C" fn Reset() -> ! {
//    let _x = 42;
//
//    // can't return so we go into an infinite loop here
//    loop {}
//}

// The reset vector, a pointer into the reset handler
//#[link_section = ".vector_table.reset_vector"]
//#[no_mangle]
//pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;
