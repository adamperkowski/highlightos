#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::structures::paging::PageTable;

use hlshell::{print, println};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use hlshell::mem;
    use x86_64::{structures::paging::Page, VirtAddr};

    #[cfg(debug_assertions)]
    println!("Initializing...\n");

    hlshell::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { mem::init(phys_mem_offset) };
    let mut frame_allocator = mem::EmptyFrameAllocator;

    let page = Page::containing_address(VirtAddr::new(0));
    mem::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    print!("\nHighlightOS Shell v0.3.0\n\nhls < ");

    hlshell::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
