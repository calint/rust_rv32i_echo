#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};

mod constants;
use constants::*;

fn uart_read_char() -> i8 {
    loop {
        unsafe {
            let input = read_volatile(UART_IN_ADDR as *const i32);
            if input == -1 {
                continue;
            }
            return input as i8;
        }
    }
}

fn uart_send_char(ch: i8) {
    unsafe {
        while read_volatile(UART_OUT_ADDR as *const i32) != -1 {}
        write_volatile(UART_OUT_ADDR as *mut i32, ch as i32);
    }
}

global_asm!(include_str!("startup.s"));

// Entry point after 'startup.s'
#[no_mangle]
pub extern "C" fn run() -> ! {
    loop {
        uart_send_char(uart_read_char());
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
