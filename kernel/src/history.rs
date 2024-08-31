use alloc::string::String;
use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;

pub struct Commands {
    pub history: Vec<String>,
    pub last: usize,
}

#[allow(clippy::new_without_default)]
impl Commands {
    pub fn new() -> Commands {
        Commands {
            history: Vec::new(),
            last: 1,
        }
    }
}

lazy_static! {
    pub static ref CMD_HISTORY: Mutex<Commands> = Mutex::new(Commands::new());
}
