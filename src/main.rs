#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};

// Hardware register definitions
#[inline(always)]
unsafe fn uart_out() -> *mut i32 {
    0xffff_fff8 as *mut i32
}

#[inline(always)]
unsafe fn uart_in() -> *mut i32 {
    0xffff_fff4 as *mut i32
}

// Assembly startup code
global_asm!(
    r#"
    .global _start
    _start:
        li sp, 0x200000
        j run
"#
);

fn uart_read_blocking() -> i8 {
    loop {
        unsafe {
            let input = read_volatile(uart_in());
            if input == -1 {
                continue;
            }
            return input as i8;
        }
    }
}

fn uart_write_blocking(ch: i8) {
    unsafe {
        while read_volatile(uart_out()) != -1 {}
        write_volatile(uart_out(), ch as i32);
    }
}

// Entry point after stack setup
#[no_mangle]
pub extern "C" fn run() -> ! {
    loop {
        uart_write_blocking(uart_read_blocking());
    }
}

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// We need to provide these language items for `no_std`
#[no_mangle]
pub extern "C" fn abort() -> ! {
    loop {}
}
