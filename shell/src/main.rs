#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]
#![warn(clippy::new_without_default)]
#![warn(clippy::missing_safety_doc)]

extern crate alloc;
use alloc::{string::String, vec};

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

use hlshell::{print, println};

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

    print!("\nHighlightOS Shell v{}\n\n", env!("CARGO_PKG_VERSION"));

    // loop {
    //     let mut input = String::new();
    //     print!("hls < ");

    //     // io::stdout().flush().unwrap();
    //     // io::stdin().read_line(&mut inpt).unwrap();

    //     // input.pop();

    //     let mut args: vec::Vec<&str> = input.split(' ').collect();

    //     if let Some(command) = COMMAND_LIST.iter().find(|&com| com.name == args[0]) {
    //         // args.remove(0);

    //         let rtr = (command.fun)(args);

    //         if rtr != 1 {
    //             if let Some(return_code) = RTR_LIST.iter().find(|&rtr_t| rtr_t.code == &rtr) {
    //                 println!("\n > {}\n{}\n", input, return_code.info);
    //             } else {
    //                 println!("\n > {}\nreturned : {}\n", input, rtr);
    //             }
    //         }
    //     } else {
    //         println!("\n > {}\ncommand not found\n", input);
    //     }
    // }

    hlshell::hlt_loop();
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
    loop {}
}
