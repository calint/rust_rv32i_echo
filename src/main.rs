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

#[inline(always)]
unsafe fn led() -> *mut u32 {
    0xffff_fffc as *mut u32
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

// Entry point after stack setup
#[no_mangle]
pub unsafe extern "C" fn run() -> ! {
    // Initialize - turn LED on to show we're running
    *led() = 0;

    loop {
        // Check for UART input
        let input = read_volatile(uart_in());

        // If we have a character (not -1)
        if input != -1 {
            // Wait until UART output is ready (not -1)
            while read_volatile(uart_out()) != -1 {}

            // Now UART is ready, send the character
            write_volatile(uart_out(), input);
        }
    }
}

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        // Flash LED rapidly to indicate panic
        let mut i = 0;
        loop {
            *led() = i & 1;
            i = i.wrapping_add(1);
        }
    }
}

// We need to provide these language items for `no_std`
#[no_mangle]
pub extern "C" fn abort() -> ! {
    loop {}
}
