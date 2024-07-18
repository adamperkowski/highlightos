#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;

use hlshell::{print, println};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    hlshell::init();

    print!("HighlightOS Shell v0.3.0\n\nhls < ");

    /* fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow(); */

    /* unsafe {
        *(0xdeadbeef as *mut u8) = 42; // trigger a page fault
    }; */

    // loop {}
    hlshell::hlt_loop();
}
