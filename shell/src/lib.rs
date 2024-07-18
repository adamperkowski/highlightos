#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

pub mod interrupts;
pub mod vga_buffer;

pub fn init() {
    interrupts::init_idt();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    loop {}
}
