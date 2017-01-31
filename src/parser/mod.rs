
macro_rules! fail {
    ($kind:expr, $msg:expr) => {
        return Err(ParseError::new($kind, $msg));
    };
}

macro_rules! map_err {
    ($result:expr, $err:expr) => {
        match $result {
            Ok(val) => Ok(val),
            Err(_) => Err($err),
        }
    };
}

pub mod types;

mod error;
pub use self::error::ParseError as Error;
