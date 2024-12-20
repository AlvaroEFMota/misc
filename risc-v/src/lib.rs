#![no_std]

use core::arch::asm;
use core::fmt::Write;

#[inline]
pub fn putchar(ch: u8) {
    unsafe {
        asm!("ecall", in("a7") SysCallType::ConsolePutchar as u8, in("a0") ch as usize);
    }
}

#[inline]
pub fn shutdown() {
    unsafe {
        asm!("ecall", in("a7") SysCallType::Shutdown as u8);
    }
}

pub struct Stdout;

impl core::fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for ch in s.bytes() {
            putchar(ch);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        let _ = write!(Stdout, $($arg)*);
    };
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        let _ = writeln!(Stdout, $($arg)*);
    };
}

enum SysCallType {
    ConsolePutchar = 1,
    //ConsoleGetchar = 2,
    Shutdown = 8,
}
