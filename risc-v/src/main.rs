#![no_std]
#![no_main]
use core::arch::asm;
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
    //let device_ptr = DEVICE_BASE_ADDRESS as *mut usize;
    //unsafe {
    //    *device_ptr = 10_usize;
    //}
    //
    //for counter in 32..=126 {
    //    write_uart(counter);
    //    write_uart(b' ');
    //}
    //
    //write_uart(b'\n');
    //write_uart(b'0');
    ////print("c");
    ////write_uart(32u8);
    ////print("!");
    //print("0123456789");
    putchar(b'H');
    putchar(b'e');
    putchar(b'l');
    putchar(b'l');
    putchar(b'o');
    putchar(b',');
    putchar(b' ');
    putchar(b'R');
    putchar(b'I');
    putchar(b'S');
    putchar(b'C');
    putchar(b'-');
    putchar(b'V');
    putchar(b'!');
    putchar(b'0');
    putchar(b'1');
    putchar(b'2');
    putchar(b'3');
    putchar(b'4');
    putchar(b'5');
    putchar(b'6');
    putchar(b'7');
    putchar(b'8');
    putchar(b'9');
    putchar(b'\n');

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
enum SysCallType {
    ConsolePutchar = 1,
    //ConsoleGetchar = 2,
    //Shutdown = 8,
}

#[inline]
pub fn putchar(ch: u8) {
    unsafe {
        asm!("ecall", in("a7") SysCallType::ConsolePutchar as usize, in("a0") ch as usize);
    }
}
