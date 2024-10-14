extern crate alloc;

use crate::gdt;
use crate::history::CMD_HISTORY;
use crate::hlt_loop;
use crate::keyboard_buffer::{clear_buffer, BUFFER, BUFFER_INDEX, BUFFER_SIZE};
use crate::print;
use crate::vga_buffer::{Color, WRITER};
use alloc::format;
use lazy_static::lazy_static;
use pc_keyboard::KeyCode;
use pic8259::ChainedPics;
use spin;
use x86_64::structures::idt::PageFaultErrorCode;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(breakpoint_handler);

        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_IDX);
        }

        idt.page_fault.set_handler_fn(page_fault_handler);

        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);

        idt
    };
}

pub const PIC_0_OFFSET: u8 = 32;
pub const PIC_1_OFFSET: u8 = PIC_0_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(unsafe { ChainedPics::new(PIC_0_OFFSET, PIC_1_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_0_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    WRITER.lock().print_colored(
        format!("\nKERNEL CRASHED\nEX: BREAKPOINT\n{:#?}\n\n", stack_frame),
        Color::Red,
        Color::Black,
    );
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("\nKERNEL CRASHED\nEX: DOUBLE FAULT\n{:#?}\n", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(Keyboard::new(
            ScancodeSet1::new(),
            layouts::Us104Key,
            HandleControl::Ignore
        ));
    }

    if unsafe { BUFFER_INDEX } < BUFFER_SIZE {
        let mut keyboard = KEYBOARD.lock();
        let mut port = Port::new(0x60);

        let scancode: u8 = unsafe { port.read() };
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => {
                        if character == '\u{8}' {
                            // backspace
                            unsafe {
                                if BUFFER_INDEX > 0 {
                                    BUFFER_INDEX -= 1;
                                    WRITER.lock().decrement_column_position();
                                    print!(" ");
                                    WRITER.lock().decrement_column_position();
                                }
                            }
                        } else {
                            unsafe {
                                BUFFER[BUFFER_INDEX] = character;
                                BUFFER_INDEX += 1;
                            }
                            print!("{}", character);
                        }
                    }

                    DecodedKey::RawKey(key) => match key {
                        KeyCode::F1 => {
                            if unsafe { BUFFER_INDEX } > 0 {
                                clear_buffer();
                                print!("\n");
                                unsafe {
                                    BUFFER[BUFFER_INDEX] = '\n';
                                    BUFFER_INDEX += 1;
                                }
                            }
                        }

                        KeyCode::ArrowUp => {
                            let mut cmd_history = CMD_HISTORY.lock();

                            if cmd_history.history.len() > cmd_history.last {
                                while unsafe { BUFFER_INDEX } > 0 {
                                    unsafe {
                                        BUFFER_INDEX -= 1;
                                        WRITER.lock().decrement_column_position();
                                        print!(" ");
                                        WRITER.lock().decrement_column_position();
                                    }
                                }

                                for i in cmd_history.history[cmd_history.history.len() - cmd_history.last - 1].chars() {
                                    unsafe {
                                        BUFFER[BUFFER_INDEX] = i;
                                        BUFFER_INDEX += 1;
                                    }
                                    print!("{}", i);
                                }
                                cmd_history.last += 1;
                            }
                        }

                        KeyCode::ArrowDown => {
                            let mut cmd_history = CMD_HISTORY.lock();

                            if cmd_history.last > 1 {
                                while unsafe { BUFFER_INDEX } > 0 {
                                    unsafe {
                                        BUFFER_INDEX -= 1;
                                        WRITER.lock().decrement_column_position();
                                        print!(" ");
                                        WRITER.lock().decrement_column_position();
                                    }
                                }

                                cmd_history.last -= 1;

                                for i in cmd_history.history[cmd_history.history.len() - cmd_history.last].chars() {
                                    unsafe {
                                        BUFFER[BUFFER_INDEX] = i;
                                        BUFFER_INDEX += 1;
                                    }
                                    print!("{}", i);
                                }
                            }
                        }

                        KeyCode::ArrowLeft => {
                            #[cfg(debug_assertions)]
                            WRITER.lock().decrement_column_position();
                        }

                        KeyCode::ArrowRight => {
                            #[cfg(debug_assertions)]
                            WRITER.lock().increment_column_position();
                        }

                        _ => {}
                    },
                }
            }
        }
    }

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
    use x86_64::registers::control::Cr2;

    WRITER.lock().print_colored(
        format!(
            "\nKERNEL CRASHED\nEX: PAGE FAULT\nAccessed address: {:?}\nError code: {:?}\n\n{:#?}\n\n",
            Cr2::read(),
            error_code,
            stack_frame
        ),
        Color::Red,
        Color::Black,
    );

    hlt_loop();
}
