
use std::fmt;

#[derive(Debug,Copy,Clone,Eq,PartialEq,Ord,PartialOrd)]
pub struct Wire {
    src_pin: u8,
    dst_pin: u8,
    dst_id: i64,
}

impl Wire {
    pub fn new(src_pin: u8, dst_pin: u8, dst_id: i64) -> Wire {
        Wire {
            src_pin: src_pin,
            dst_pin: dst_pin,
            dst_id: dst_id,
        }
    }

    pub fn src_pin(&self) -> u8 {
        self.src_pin
    }

    pub fn dst_pin(&self) -> u8 {
        self.dst_pin
    }

    pub fn dst_gate(&self) -> i64 {
        self.dst_id
    }

    pub fn is_output(self) -> bool {
        self.dst_id < 0
    }
}

impl fmt::Display for Wire {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.src_pin, self.dst_id, self.dst_pin)
    }
}
