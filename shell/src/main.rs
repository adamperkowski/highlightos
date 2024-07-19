#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

extern crate alloc;

use alloc::boxed::Box;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::structures::paging::PageTable;

use hlshell::{print, println};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use hlshell::allocator;
    use hlshell::mem::{self, BootInfoFrameAlloc};
    use x86_64::{structures::paging::Page, VirtAddr};

    #[cfg(debug_assertions)]
    println!("Initializing...\n");

    hlshell::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { mem::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAlloc::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap initialization failed");

    let x = Box::new(41);

    print!(
        "\nHighlightOS Shell v{}\n\nhls < ",
        env!("CARGO_PKG_VERSION")
    );

    hlshell::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
