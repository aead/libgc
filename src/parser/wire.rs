
use super::gate::{Pin, ID};

use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Wire {
    src_pin: Pin,
    dst_pin: Pin,
    dst_id: ID,
}

impl Wire {
    pub fn new(src_pin: Pin, dst_pin: Pin, dst_id: ID) -> Wire {
        Wire {
            src_pin: src_pin,
            dst_pin: dst_pin,
            dst_id: dst_id,
        }
    }

    #[inline]
    pub fn src_pin(&self) -> Pin {
        self.src_pin
    }

    #[inline]
    pub fn dst_pin(&self) -> Pin {
        self.dst_pin
    }

    #[inline]
    pub fn dst_gate(&self) -> ID {
        self.dst_id
    }

    #[inline]
    pub fn is_output(&self) -> bool {
        match self.dst_id {
            ID::Output(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_input(&self) -> bool {
        match self.dst_id {
            ID::Input(_) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Wire {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.src_pin, self.dst_id, self.dst_pin)
    }
}
