#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

pub mod gdt;
pub mod interrupts;
pub mod vga_buffer;

pub fn init() {
    interrupts::init_idt();
    gdt::init();
}
