#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]
#![warn(clippy::new_without_default)]
#![warn(clippy::missing_safety_doc)]

extern crate alloc;
use alloc::{format, vec};

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

use hlkernel::{
    history::CMD_HISTORY,
    keyboard_buffer, print, println,
    vga_buffer::{Color, WRITER},
};

mod cmd;
use cmd::COMMAND_LIST;

// Add RTC module
mod rtc;

// Global variables
static mut ACPI_CONTROLLER: Option<AcpiController> = None;
static mut RTC_CONTROLLER: Option<rtc::Rtc> = None;

entry_point!(kernel_main);

struct RtrType {
    code: &'static i32,
    info: &'static str,
    color: Color,
}

// Simple structure for ACPI
struct AcpiController;

impl AcpiController {
    pub unsafe fn new() -> Self {
        Self
    }
    
    pub fn shutdown(&mut self) {
        // Call the public shutdown function
        cmd::shutdown_command(vec![]);
    }
    
    pub fn reboot(&mut self) {
        // Call the public reboot function
        cmd::reboot_command(vec![]);
    }
}

pub fn started_colors() {
    WRITER.lock().print_colored(
        format!("\naA"),
        Color::Blue,
        Color::Blue,
    );
    WRITER.lock().print_colored(
        format!("aA"),
        Color::Pink,
        Color::Pink,
    );
    WRITER.lock().print_colored(
        format!("aA"),
        Color::Red,
        Color::Red,
    );
    WRITER.lock().print_colored(
        format!("aA"),
        Color::Green,
        Color::Green,
    );
    WRITER.lock().print_colored(
        format!("aA"),
        Color::Yellow,
        Color::Yellow,
    );
    WRITER.lock().print_colored(
        format!("aA"),
        Color::LightBlue,
        Color::LightBlue,
    );

    WRITER.lock().print_colored(
        format!("aA"),
        Color::Magenta,
        Color::Magenta,
    );

    WRITER.lock().print_colored(
        format!("aA"),
        Color::Cyan,
        Color::Cyan,
    );

    WRITER.lock().print_colored(
        format!("aA"),
        Color::Brown,
        Color::Brown,
    );

    WRITER.lock().print_colored(
        format!("aA"),
        Color::Blue,
        Color::Blue,
    );

    WRITER.lock().print_colored(
        format!("aA"),
        Color::Blue,
        Color::Blue,
    );
}

pub fn init_kernel(boot_info: &'static BootInfo) {
    use hlkernel::allocator;
    use hlkernel::mem::{self, BootInfoFrameAlloc};
    use x86_64::VirtAddr;

    #[cfg(debug_assertions)]
    println!("Initializing...\n");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { mem::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAlloc::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap initialization failed");

    hlkernel::init();

    // Initialize ACPI controller
    unsafe {
        ACPI_CONTROLLER = Some(AcpiController::new());
    }

    // Initialize RTC
    unsafe {
        RTC_CONTROLLER = Some(rtc::Rtc::new());
    }

    #[cfg(debug_assertions)]
    WRITER.lock().print_colored(
        format!("\nHighlightOS v{} *DEBUG*", env!("CARGO_PKG_VERSION")),
        Color::Black,
        Color::Yellow,
    );

    #[cfg(not(debug_assertions))]
    WRITER.lock().print_colored(
        format!("\n                 HighlightOS v{}", env!("CARGO_PKG_VERSION")),
        Color::Black,
        Color::Yellow,
    );
    WRITER.lock().print_colored(
        format!("\n Documentation: https://os.adamperkowski.dev"),
        Color::Cyan,
        Color::Black
    );

    print!("\n\nhls > ");
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    init_kernel(boot_info);

    loop {
        let input = keyboard_buffer::read_buffer();

        if input.ends_with("\n") {
            keyboard_buffer::clear_buffer();
            CMD_HISTORY.lock().last = 0;

            let mut args: vec::Vec<&str> = input.split(' ').collect();

            if args[0] != "\n" {
                let req_com = &args[0].replace("\n", "");

                if let Some(command) = COMMAND_LIST.iter().find(|&com| com.name == req_com) {
                    args.remove(0);

                    print!("\n");

                    let rtr = (command.fun)(args);

                    if rtr != 1 {
                        if let Some(return_code) = RTR_LIST.iter().find(|&rtr_t| rtr_t.code == &rtr) {
                            println!("\n > {}", req_com);
                            WRITER.lock().print_colored(
                                format!("{}:{}\n\n", rtr, return_code.info),
                                return_code.color,
                                Color::Black,
                            );
                        } else {
                            println!("\n > {}\nreturned : {}\n", req_com, rtr);
                        }
                    }
                } else {
                    WRITER.lock().print_colored(
                        format!("\n > hls: command not found: {}\n", input),
                        Color::LightRed,
                        Color::Black,
                    );
                }

                let mut cmd_history = CMD_HISTORY.lock();
                if !cmd_history.history.is_empty() {
                    if cmd_history.history[cmd_history.history.len() - 1] != input.replace("\n", "") {
                        cmd_history.history.push(input.replace("\n", ""));
                    }
                } else {
                    cmd_history.history.push(input.replace("\n", ""));
                }
            }

            print!("hls > ");
        }
    }
}

const RTR_LIST: &[RtrType] = &[
    RtrType {
        code: &0,
        info: "executed successfully",
        color: Color::Green,
    },
    RtrType {
        code: &2,
        info: "returned general error",
        color: Color::Red,
    },
    RtrType {
        code: &3,
        info: "returned critical error",
        color: Color::Red,
    },
    RtrType {
        code: &4,
        info: "returned user error",
        color: Color::Red,
    },
];

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    WRITER
        .lock()
        .print_colored(format!("KERNEL CRASHED\n{}\n", info), Color::Red, Color::Black);
    hlkernel::hlt_loop();
}
