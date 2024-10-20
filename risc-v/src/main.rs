#![no_std]
#![no_main]
use core::panic::PanicInfo;

//const DEVICE_BASE_ADDRESS: usize = 0xff200020;
const DEVICE_BASE_ADDRESS: usize = 0x80000000;
const UART0_BASE: usize = 0x10000000;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn write_uart(ch: u8) {
    let uart = UART0_BASE as *mut u8;
    unsafe {
        //core::ptr::write_volatile(uart, ch);
        uart.write_volatile(ch)
    }
}

fn print(s: &str) {
    for byte in s.bytes() {
        write_uart(byte);
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let device_ptr = DEVICE_BASE_ADDRESS as *mut usize;
    unsafe {
        *device_ptr = 10_usize;
    }

    for counter in 32..=126 {
        write_uart(counter);
        write_uart(b' ');
    }

    write_uart(b'\n');
    write_uart(b'0');
    print("c");
    //write_uart(32u8);
    //print("!");
    print("0123456789");

    loop {
        //write_uart(100u8);
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
