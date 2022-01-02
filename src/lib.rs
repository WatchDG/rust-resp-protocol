use bytes::Bytes;

mod array;
mod bulk_string;
mod error;
mod integer;
mod simple_string;

pub use array::{Array, ArrayBuilder, EMPTY_ARRAY, NULL_ARRAY};
pub use bulk_string::{BulkString, EMPTY_BULK_STRING, NULL_BULK_STRING};
pub use error::Error;
pub use integer::Integer;
pub use simple_string::SimpleString;

#[derive(Debug, Clone)]
pub enum RespError {
    InvalidFirstChar,
    InvalidLength,
    InvalidLengthSeparator,
    InvalidValue,
    InvalidTerminate,
    LengthsNotMatch,
}

impl std::fmt::Display for RespError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RespError::InvalidFirstChar => {
                write!(f, "Invalid first char.")
            }
            RespError::InvalidLength => {
                write!(f, "Invalid length.")
            }
            RespError::InvalidLengthSeparator => {
                write!(f, "Invalid length separator.")
            }
            RespError::InvalidValue => {
                write!(f, "Invalid value.")
            }
            RespError::LengthsNotMatch => {
                write!(f, "Lengths do not match.")
            }
            RespError::InvalidTerminate => {
                write!(f, "Invalid terminate.")
            }
        }
    }
}

impl std::error::Error for RespError {}

#[derive(Debug, Clone)]
pub enum RespType {
    SimpleString(SimpleString),
    Error(Error),
    Integer(Integer),
    BulkString(BulkString),
    Array(Array),
}

impl RespType {
    fn len(&self) -> usize {
        match self {
            RespType::SimpleString(simple_string) => simple_string.len(),
            RespType::Error(error) => error.len(),
            RespType::Integer(integer) => integer.len(),
            RespType::BulkString(bulk_string) => bulk_string.len(),
            RespType::Array(array) => array.len(),
        }
    }

    fn bytes(&self) -> Bytes {
        match self {
            RespType::SimpleString(simple_string) => simple_string.bytes(),
            RespType::Error(error) => error.bytes(),
            RespType::Integer(integer) => integer.bytes(),
            RespType::BulkString(bulk_string) => bulk_string.bytes(),
            RespType::Array(array) => array.bytes(),
        }
    }
}
