// src/rtc.rs
use x86_64::instructions::port::Port;

const RTC_ADDR: u16 = 0x70;
const RTC_DATA: u16 = 0x71;

pub struct Rtc {
    addr_port: Port<u8>,
    data_port: Port<u8>,
}

#[derive(Debug, Clone, Copy)]
pub struct DateTime {
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
    pub day: u8,
    pub month: u8,
    pub year: u8,
}

impl Rtc {
    pub unsafe fn new() -> Self {
        Self {
            addr_port: Port::new(RTC_ADDR),
            data_port: Port::new(RTC_DATA),
        }
    }

    // Read from an RTC register
    unsafe fn read_register(&mut self, reg: u8) -> u8 {
        self.addr_port.write(reg);
        self.data_port.read()
    }

    // Wait for RTC to not be updating
    unsafe fn wait_for_update(&mut self) {
        while self.read_register(0x0A) & 0x80 != 0 {}
    }

    // Read current date and time
    pub unsafe fn read_datetime(&mut self) -> DateTime {
        self.wait_for_update();

        let second = self.read_register(0x00);
        let minute = self.read_register(0x02);
        let hour = self.read_register(0x04);
        let day = self.read_register(0x07);
        let month = self.read_register(0x08);
        let year = self.read_register(0x09);

        // Convert from BCD to binary if necessary
        let register_b = self.read_register(0x0B);
        let is_bcd = register_b & 0x04 == 0;

        DateTime {
            second: if is_bcd { self.bcd_to_bin(second) } else { second },
            minute: if is_bcd { self.bcd_to_bin(minute) } else { minute },
            hour: if is_bcd { self.bcd_to_bin(hour) } else { hour },
            day: if is_bcd { self.bcd_to_bin(day) } else { day },
            month: if is_bcd { self.bcd_to_bin(month) } else { month },
            year: if is_bcd { self.bcd_to_bin(year) } else { year },
        }
    }

    // Convert BCD to binary
    fn bcd_to_bin(&self, bcd: u8) -> u8 {
        ((bcd >> 4) * 10) + (bcd & 0x0F)
    }
}

impl DateTime {
    pub fn format_time(&self) -> alloc::string::String {
        alloc::format!("{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }

    pub fn format_date(&self) -> alloc::string::String {
        alloc::format!("{:02}/{:02}/20{:02}", self.day, self.month, self.year)
    }

    pub fn format_full(&self) -> alloc::string::String {
        alloc::format!("{} {}", self.format_date(), self.format_time())
    }
}
