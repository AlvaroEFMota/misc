#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr;

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
    //let device_ptr = DEVICE_BASE_ADDRESS as *mut u8;
    let device_ptr = DEVICE_BASE_ADDRESS as *mut u32;

    unsafe {
        ptr::write_volatile(device_ptr, num_to_display(1234));
        ptr::write_volatile(device_ptr, num_to_display(-123));
    }

    unsafe {
        for num in display_numbers {
            //*device_ptr = black_box(num);
            ptr::write_volatile(device_ptr, num);
        }
        let device_ptr = device_ptr.add(1);
        for num in display_numbers {
            ptr::write_volatile(device_ptr, num);
        }
        let device_ptr = device_ptr.add(1);
        for num in display_numbers {
            ptr::write_volatile(device_ptr, num);
        }
        let device_ptr = device_ptr.add(1);
        for num in display_numbers {
            ptr::write_volatile(device_ptr, num);
        }
    }

    loop {}
}

fn num_to_display(mut num: i32) -> u32 {
    let num_disp = [0x3f, 0x06, 0x5b, 0x4f, 0x66, 0x6d, 0x7d, 0x7f, 0x6f];
    let mut result = 0u32;
    let delimiter = if num > 0 {
        4
    } else {
        result += 0x40 << (8 * 3);
        num *= -1;
        3
    };
    for i in (0..delimiter).rev() {
        let tmp = num / 10_i32.pow(i);
        if tmp > 0 {
            result += num_disp[tmp as usize] << (8 * i);
        }
        num -= tmp * 10_i32.pow(i);
    }
    result
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
