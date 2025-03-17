#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};

mod constants; // FPGA addresses

// API
fn uart_read_char() -> u8 {
    loop {
        unsafe {
            let input = read_volatile(constants::UART_IN_ADDR as *const i32);
            if input == -1 {
                continue;
            }
            return input as u8;
        }
    }
}

fn uart_send_char(ch: u8) {
    unsafe {
        while read_volatile(constants::UART_OUT_ADDR as *const i32) != -1 {}
        write_volatile(constants::UART_OUT_ADDR as *mut i32, ch as i32);
    }
}

fn uart_send_cstr(cstr: *const u8) {
    unsafe {
        let mut ptr = cstr;
        while *ptr != 0 {
            while read_volatile(constants::UART_OUT_ADDR as *const i32) != -1 {}
            write_volatile(constants::UART_OUT_ADDR as *mut i32, *ptr as i32);
            ptr = ptr.offset(1);
        }
    }
}

// setup stack and jump to 'run()'
global_asm!(include_str!("startup.s"));

#[no_mangle]
pub extern "C" fn run() -> ! {
    uart_send_cstr(b"hello world. echo:\r\n\0".as_ptr());
    loop {
        uart_send_char(uart_read_char());
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
