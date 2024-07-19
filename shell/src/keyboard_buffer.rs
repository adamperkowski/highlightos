use alloc::string::String;

pub const BUFFER_SIZE: usize = 256;
pub static mut BUFFER: [char; BUFFER_SIZE] = ['\0'; BUFFER_SIZE];
pub static mut BUFFER_INDEX: usize = 0;

pub fn read_buffer() -> String {
    let mut buffer_content = String::new();

    unsafe {
        for i in 0..BUFFER_INDEX {
            buffer_content.push(BUFFER[i]);
        }
    }

    buffer_content
}

pub fn clear_buffer() {
    unsafe {
        for i in 0..BUFFER_SIZE {
            BUFFER[i] = '\0';
        }
        BUFFER_INDEX = 0;
    }
}
