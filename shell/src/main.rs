#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]
#![warn(clippy::new_without_default)]
#![warn(clippy::missing_safety_doc)]

extern crate alloc;
use alloc::vec;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

use hlshell::{keyboard_buffer, print, println};

mod cmd;
use cmd::COMMAND_LIST;

entry_point!(kernel_main);

struct RtrType {
    code: &'static i32,
    info: &'static str,
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use hlshell::allocator;
    use hlshell::mem::{self, BootInfoFrameAlloc};
    use x86_64::VirtAddr;

    #[cfg(debug_assertions)]
    println!("Initializing...\n");

    hlshell::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { mem::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAlloc::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap initialization failed");

    // let args: vec::Vec<&str> = vec![""];
    // (COMMAND_LIST[1].fun)(args);

    print!(
        "\nHighlightOS Shell v{}\n\nhls < ",
        env!("CARGO_PKG_VERSION")
    );

    loop {
        let input = keyboard_buffer::read_buffer();

        if input.ends_with("\n") {
            keyboard_buffer::clear_buffer();

            let mut args: vec::Vec<&str> = input.split(' ').collect();

            if args[0] != "\n" {
                let req_com = &args[0].replace("\n", "");

                if let Some(command) = COMMAND_LIST.iter().find(|&com| com.name == req_com) {
                    args.remove(0);

                    let rtr = (command.fun)(args);

                    if rtr != 1 {
                        if let Some(return_code) = RTR_LIST.iter().find(|&rtr_t| rtr_t.code == &rtr)
                        {
                            println!("\n > {}\n{}\n", req_com, return_code.info);
                        } else {
                            println!("\n > {}\nreturned : {}\n", req_com, rtr);
                        }
                    }
                } else {
                    println!("\n > {}\ncommand not found\n", input);
                }
            }

            print!("hls < ");
        }
    }

    // hlshell::hlt_loop();
}

const RTR_LIST: &[RtrType] = &[
    RtrType {
        code: &0,
        info: "executed successfully",
    },
    RtrType {
        code: &2,
        info: "returned general error",
    },
    RtrType {
        code: &3,
        info: "returned critical error",
    },
    RtrType {
        code: &4,
        info: "returned user error",
    },
];

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlshell::hlt_loop();
}
