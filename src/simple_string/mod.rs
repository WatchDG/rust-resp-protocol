use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum SimpleStringError {
    InvalidValueChar,
    InvalidFirstChar,
    InvalidTerminate,
}

impl fmt::Display for SimpleStringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SimpleStringError::InvalidValueChar => {
                write!(f, "[SimpleStringError] Invalid value char.")
            }
            SimpleStringError::InvalidFirstChar => {
                write!(f, "[SimpleStringError] Invalid first char.")
            }
            SimpleStringError::InvalidTerminate => {
                write!(f, "[SimpleStringError] Invalid terminate.")
            }
        }
    }
}

impl Error for SimpleStringError {}

#[derive(Debug, PartialEq)]
pub struct SimpleString(Bytes);

/// Simple string type
impl SimpleString {
    /// Build a new Simple string
    ///
    /// # Example
    /// ```
    /// use resp_protocol::SimpleString;
    ///
    /// let simple_string = SimpleString::new(b"OK");
    /// ```
    #[inline]
    pub fn new(value: &[u8]) -> SimpleString {
        let mut bytes = BytesMut::with_capacity(value.len() + 3);
        bytes.put_u8(0x2b); // "+"
        bytes.put_slice(value);
        bytes.put_u8(0x0d); // CR
        bytes.put_u8(0x0a); // LF
        SimpleString(bytes.freeze())
    }

    #[inline]
    pub fn bytes(&self) -> Bytes {
        self.0.clone()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn value(&self) -> Vec<u8> {
        let length = self.len();
        let mut bytes = self.0.slice(1..(length - 2));
        let mut vector = Vec::<u8>::with_capacity(length - 3);
        unsafe {
            vector.set_len(length - 3);
        }
        bytes.copy_to_slice(vector.as_mut_slice());
        vector
    }

    #[inline]
    pub fn value_len(&self) -> usize {
        self.len() - 3
    }

    pub fn validate_value(input: &[u8]) -> Result<(), SimpleStringError> {
        let mut index = 0;
        let length = input.len();
        while index < length && input[index] != 0x0d && input[index] != 0x0a {
            index += 1;
        }
        if index != length {
            return Err(SimpleStringError::InvalidValueChar);
        }
        Ok(())
    }

    pub fn parse(
        input: &[u8],
        start: &mut usize,
        end: &usize,
    ) -> Result<SimpleString, SimpleStringError> {
        let mut index = *start;
        if input[index] != 0x2b {
            return Err(SimpleStringError::InvalidFirstChar);
        }
        index += 1;
        while index < *end && input[index] != 0x0d && input[index] != 0x0a {
            index += 1;
        }
        if index + 1 >= *end || input[index] != 0x0d || input[index + 1] != 0x0a {
            return Err(SimpleStringError::InvalidTerminate);
        }
        let value = Self::new(&input[(*start + 1)..index]);
        *start = index + 2;
        Ok(value)
    }
}

#[cfg(test)]
mod tests_simple_string {
    use crate::simple_string::SimpleString;
    use bytes::Bytes;

    #[test]
    fn test_new() {
        let string = "OK";
        let simple_string = SimpleString::new(string.as_bytes());
        assert_eq!(simple_string, SimpleString(Bytes::from_static(b"+OK\r\n")));
    }

    #[test]
    fn test_value() {
        let simple_string = SimpleString(Bytes::from_static(b"+OK\r\n"));
        assert_eq!(simple_string.value(), Vec::from("OK"));
        assert_eq!(simple_string.value(), Vec::from("OK"));
    }

    #[test]
    fn test_value_len() {
        let simple_string = SimpleString(Bytes::from_static(b"+OK\r\n"));
        assert_eq!(simple_string.value_len(), 2);
        assert_eq!(simple_string.value_len(), 2);
    }

    #[test]
    fn test_bytes() {
        let simple_string = SimpleString(Bytes::from_static(b"+OK\r\n"));
        assert_eq!(simple_string.bytes(), Bytes::from_static(b"+OK\r\n"));
        assert_eq!(simple_string.bytes(), Bytes::from_static(b"+OK\r\n"));
    }

    #[test]
    fn test_len() {
        let simple_string = SimpleString(Bytes::from_static(b"+OK\r\n"));
        assert_eq!(simple_string.len(), 5);
        assert_eq!(simple_string.len(), 5);
    }

    #[test]
    fn test_validate_valid_value() {
        let value = b"OK";
        assert_eq!(SimpleString::validate_value(value).unwrap(), ())
    }

    #[test]
    #[should_panic(expected = "InvalidValueChar")]
    fn test_validate_invalid_value() {
        let value = b"O\r\nK";
        assert_eq!(SimpleString::validate_value(value).unwrap(), ())
    }

    #[test]
    fn test_parse() {
        let string = "+foo\r\n+bar\r\n";
        let mut cursor = 0;
        let end = string.len();
        assert_eq!(
            SimpleString::parse(string.as_bytes(), &mut cursor, &end).unwrap(),
            SimpleString::new("foo".as_bytes())
        );
        assert_eq!(cursor, 6);
    }
}
