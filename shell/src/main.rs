#![feature(global_asm)]
#![no_main]
#![no_std]

global_asm!(r#"
loop:
  jmp loop
.org 510
.word 0xaa55
"# :::: "volatile");

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
