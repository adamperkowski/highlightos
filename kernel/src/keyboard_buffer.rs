use alloc::string::String;

pub const BUFFER_SIZE: usize = 256;
pub static mut BUFFER: [char; BUFFER_SIZE] = ['\0'; BUFFER_SIZE];
pub static mut BUFFER_INDEX: usize = 0;

#[allow(static_mut_refs)] // TODO
pub fn read_buffer() -> String {
    let mut buffer_content = String::new();

    unsafe {
        for i in BUFFER.iter().take(BUFFER_INDEX) {
            buffer_content.push(*i);
        }
    }

    buffer_content
}

#[allow(static_mut_refs)] // TODO
pub fn clear_buffer() {
    unsafe {
        for ref mut i in BUFFER.iter().take(BUFFER_SIZE) {
            *i = &'\0';
        }
        BUFFER_INDEX = 0;
    }
}
