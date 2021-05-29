pub mod de;
pub mod error;
pub mod ser;

pub use de::{from_buf_reader, from_string};
pub use error::{Error, Result};
pub use ser::to_string;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialOrd, PartialEq)]
pub enum RESPType {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Option<Vec<u8>>),
    Array(Option<Vec<RESPType>>),
}

impl Display for RESPType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            RESPType::SimpleString(s) => write!(f, "{}", s)?,
            RESPType::Error(err) => write!(f, "{}", err)?,
            RESPType::Integer(int) => write!(f, "{}", int)?,
            RESPType::BulkString(possible_str) => match possible_str.clone() {
                Some(s) => write!(f, "{}", String::from_utf8(s).unwrap_or_default())?,
                None => write!(f, "{:?}", None::<RESPType>)?,
            },
            RESPType::Array(possible_arr) => match possible_arr {
                Some(arr) => write!(f, "{:?}", arr)?,
                None => write!(f, "{:?}", None::<RESPType>)?,
            },
        }
        Ok(())
    }
}

pub type RESP = RESPType;
