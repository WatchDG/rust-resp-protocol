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

#[derive(Debug, Clone, PartialEq)]
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

    #[inline]
    pub fn from_bytes(input: Bytes) -> Self {
        Self(input)
    }

    #[inline]
    pub fn from_slice(input: &[u8]) -> Self {
        let bytes = Bytes::copy_from_slice(input);
        Self::from_bytes(bytes)
    }

    /// Build as new Simple String from raw pointer
    ///
    /// # Example
    /// ```
    /// use resp_protocol::SimpleString;
    /// use std::mem::ManuallyDrop;
    ///
    /// let string: String = "+OK\r\n".to_owned();
    /// let mut mdrop_string: ManuallyDrop<String> = ManuallyDrop::new(string);
    /// let simple_string: SimpleString = unsafe { SimpleString::from_raw(mdrop_string.as_mut_ptr(), mdrop_string.len()) };
    /// ```
    #[inline]
    pub unsafe fn from_raw(ptr: *mut u8, length: usize) -> Self {
        let vector = Vec::from_raw_parts(ptr, length, length);
        let bytes = Bytes::from(vector);
        Self::from_bytes(bytes)
    }

    pub fn parse(
        input: &[u8],
        start: &mut usize,
        end: &usize,
    ) -> Result<SimpleString, SimpleStringError> {
        let mut index = *start;
        if index >= *end || input[index] != 0x2b {
            return Err(SimpleStringError::InvalidFirstChar);
        }
        index += 1;
        while index < *end && input[index] != 0x0d && input[index] != 0x0a {
            index += 1;
        }
        if index + 1 >= *end || input[index] != 0x0d || input[index + 1] != 0x0a {
            return Err(SimpleStringError::InvalidTerminate);
        }
        index += 2;
        let value = Self::from_slice(&input[*start..index]);
        *start = index;
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
