#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;

use hlshell::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("HighlightOS Shell v0.3.0\n\nhls < ");

    hlshell::init();

    x86_64::instructions::interrupts::int3(); //invoke a new interruption

    println!("WORKS.");

    loop {}
}
