#![no_std]
#![no_main]

use core::hint::black_box;
use core::panic::PanicInfo;

//extern crate alloc;
//use alloc::vec;
//use linked_list_allocator::LockedHeap;

//#[global_allocator]
//static ALLOCATOR: LockedHeap = LockedHeap::empty();

const DEVICE_BASE_ADDRESS: usize = 0xff200020;
//const DEVICE_BASE_ADDRESS: usize = 0x80000000;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    //static mut HEAP: [u8; 1024] = [0; 1024]; // Ajuste o tamanho conforme necessário
    //unsafe {
    //    ALLOCATOR.lock().init(HEAP.as_ptr() as usize, 1024);
    //}

    let display_numbers = [0x3f, 0x06, 0x5b, 0x4f, 0x66, 0x6d, 0x7d, 0x7f, 0x6f];
    let device_ptr = DEVICE_BASE_ADDRESS as *mut u8;

    unsafe {
        for num in display_numbers {
            *device_ptr = black_box(num);
        }
        let device_ptr = device_ptr.add(1);
        for num in display_numbers {
            *device_ptr = black_box(num);
        }
        let device_ptr = device_ptr.add(1);
        for num in display_numbers {
            *device_ptr = black_box(num);
        }
        let device_ptr = device_ptr.add(1);
        for num in display_numbers {
            *device_ptr = black_box(num);
        }
    }

    loop {}
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
