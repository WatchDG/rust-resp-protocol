use bytes::{Buf, BufMut, Bytes};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum IntegerError {
    InvalidValueChar,
    InvalidValue,
}

impl fmt::Display for IntegerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IntegerError::InvalidValueChar => {
                write!(f, "[IntegerError] Invalid value char.")
            }
            IntegerError::InvalidValue => {
                write!(f, "[IntegerError] Invalid value.")
            }
        }
    }
}

impl Error for IntegerError {}

#[derive(Debug, PartialEq)]
pub struct Integer(Bytes);

impl Integer {
    #[inline]
    pub fn new(input: i64) -> Integer {
        let string = input.to_string();
        let mut vector = Vec::with_capacity(string.len() + 3);
        vector.put_u8(0x3a); // ":"
        vector.put_slice(string.as_bytes());
        vector.put_u8(0x0d); // CR
        vector.put_u8(0x0a); // LF
        let bytes = Bytes::from(vector);
        Integer(bytes)
    }

    #[inline]
    pub fn raw_value(&self) -> Vec<u8> {
        let length = self.0.len();
        let mut bytes = self.0.slice(1..(length - 2));
        let mut vector = Vec::<u8>::with_capacity(length - 3);
        unsafe {
            vector.set_len(length - 3);
        }
        bytes.copy_to_slice(vector.as_mut_slice());
        vector
    }

    #[inline]
    pub fn bytes(&self) -> Bytes {
        self.0.clone()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn validate_value(input: &[u8]) -> Result<(), IntegerError> {
        let mut index = 0;
        let length = input.len();
        while index < length && input[index] != 0x0a && input[index] != 0x0d {
            index += 1;
        }
        if index != length {
            return Err(IntegerError::InvalidValueChar);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests_integer {
    use crate::integer::Integer;
    use bytes::Bytes;

    #[test]
    fn test_new() {
        let integer = Integer::new(100);
        assert_eq!(integer, Integer(Bytes::from_static(b":100\r\n")));
    }

    #[test]
    fn test_raw_value() {
        let integer = Integer(Bytes::from_static(b":100\r\n"));
        assert_eq!(integer.raw_value(), Vec::from("100"));
        assert_eq!(integer.raw_value(), Vec::from("100"));
    }

    #[test]
    fn test_bytes() {
        let integer = Integer(Bytes::from_static(b":100\r\n"));
        assert_eq!(integer.bytes(), Bytes::from_static(b":100\r\n"));
        assert_eq!(integer.bytes(), Bytes::from_static(b":100\r\n"));
    }

    #[test]
    fn test_validate_valid_value() {
        let value = 100i64.to_string();
        assert_eq!(Integer::validate_value(value.as_bytes()).unwrap(), ())
    }

    #[test]
    #[should_panic(expected = "InvalidValueChar")]
    fn test_validate_invalid_value() {
        let value = b"100\r\n";
        assert_eq!(Integer::validate_value(value).unwrap(), ())
    }
}
