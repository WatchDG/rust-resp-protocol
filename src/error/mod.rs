use bytes::{Buf, BufMut, Bytes};
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum ErrorError {
    InvalidValueChar,
    InvalidFirstChar,
    InvalidTerminate,
}

impl fmt::Display for ErrorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorError::InvalidValueChar => {
                write!(f, "[ErrorError] Invalid value char.")
            }
            ErrorError::InvalidFirstChar => {
                write!(f, "[ErrorError] Invalid first char.")
            }
            ErrorError::InvalidTerminate => {
                write!(f, "[ErrorError] Invalid terminate.")
            }
        }
    }
}

impl error::Error for ErrorError {}

#[derive(Debug, Clone, PartialEq)]
pub struct Error(Bytes);

/// Error type
impl Error {
    /// Build a new Error
    ///
    /// # Example
    /// ```
    /// use resp_protocol::Error;
    ///
    /// let error = Error::new(b"Invalid type.");
    /// ```
    #[inline]
    pub fn new(input: &[u8]) -> Error {
        let mut vector = Vec::with_capacity(input.len() + 3);
        vector.put_u8(0x2d); // "-"
        vector.put_slice(input);
        vector.put_u8(0x0d); // CR
        vector.put_u8(0x0a); // LF
        let bytes = Bytes::from(vector);
        Error(bytes)
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
        let length = self.0.len();
        let mut bytes = self.0.slice(1..(length - 2));
        let mut vector = Vec::<u8>::with_capacity(length - 3);
        unsafe {
            vector.set_len(length - 3);
        }
        bytes.copy_to_slice(vector.as_mut_slice());
        vector
    }

    pub fn validate_value(input: &[u8]) -> Result<(), ErrorError> {
        let mut index = 0;
        let length = input.len();
        while index < length && input[index] != 0x0d && input[index] != 0x0a {
            index += 1;
        }
        if index != length {
            return Err(ErrorError::InvalidValueChar);
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

    #[inline]
    pub unsafe fn from_raw(ptr: *mut u8, length: usize) -> Self {
        let vector = Vec::from_raw_parts(ptr, length, length);
        let bytes = Bytes::from(vector);
        Self::from_bytes(bytes)
    }

    pub fn parse(input: &[u8], start: &mut usize, end: &usize) -> Result<Error, ErrorError> {
        let mut index = *start;
        if index >= *end || input[index] != 0x2d {
            return Err(ErrorError::InvalidFirstChar);
        }
        index += 1;
        while index < *end && input[index] != 0x0d && input[index] != 0x0a {
            index += 1;
        }
        if index + 1 >= *end || input[index] != 0x0d || input[index + 1] != 0x0a {
            return Err(ErrorError::InvalidTerminate);
        }
        index += 2;
        let value = Self::from_slice(&input[*start..index]);
        *start = index;
        Ok(value)
    }
}

#[cfg(test)]
mod tests_error {
    use crate::error::Error;
    use bytes::Bytes;

    #[test]
    fn test_new() {
        let string = "Error message";
        let error = Error::new(string.as_bytes());
        assert_eq!(error, Error(Bytes::from_static(b"-Error message\r\n")));
    }

    #[test]
    fn test_value() {
        let error = Error(Bytes::from_static(b"-Error message\r\n"));
        assert_eq!(error.value(), Vec::from("Error message"));
        assert_eq!(error.value(), Vec::from("Error message"));
    }

    #[test]
    fn test_bytes() {
        let error = Error(Bytes::from_static(b"-Error message\r\n"));
        assert_eq!(error.bytes(), Bytes::from_static(b"-Error message\r\n"));
        assert_eq!(error.bytes(), Bytes::from_static(b"-Error message\r\n"));
    }

    #[test]
    fn test_validate_valid_value() {
        let value = b"Error message";
        assert_eq!(Error::validate_value(value).unwrap(), ())
    }

    #[test]
    #[should_panic(expected = "InvalidValueChar")]
    fn test_validate_invalid_value() {
        let value = b"Error\r\n message";
        assert_eq!(Error::validate_value(value).unwrap(), ())
    }

    #[test]
    fn test_parse() {
        let string = "-invalid length\r\n+bar\r\n";
        let mut cursor = 0;
        let end = string.len();
        assert_eq!(
            Error::parse(string.as_bytes(), &mut cursor, &end).unwrap(),
            Error::new("invalid length".as_bytes())
        );
        assert_eq!(cursor, 17);
    }
}
