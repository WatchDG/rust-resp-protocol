use bytes::Bytes;

pub mod array;
pub mod bulk_string;
pub mod error;
pub mod integer;
pub mod simple_string;

pub use array::{Array, ArrayBuilder, EMPTY_ARRAY, NULL_ARRAY};
pub use bulk_string::BulkString;
pub use error::{Error, ErrorError};
pub use integer::{Integer, IntegerError};
pub use simple_string::{SimpleString, SimpleStringError};

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
