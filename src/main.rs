#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};

mod constants; // Declare the constants module
use constants::*; // Bring constants into scope

// Hardware register definitions
#[inline(always)]
unsafe fn uart_out() -> *mut i32 {
    UART_OUT_ADDR as *mut i32
}

#[inline(always)]
unsafe fn uart_in() -> *mut i32 {
    UART_IN_ADDR as *mut i32
}

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

global_asm!(include_str!("startup.s"));

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
