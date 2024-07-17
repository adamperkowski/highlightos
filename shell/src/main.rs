#![no_main]
#![no_std]
#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("HighlightOS Shell v0.3.0\n\nhls < ");

    loop {}
}
