use bytes::{BufMut, Bytes, BytesMut};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum BulkStringError {
    InvalidValue,
    InvalidFirstChar,
    InvalidLength,
    InvalidLengthSeparator,
    InvalidTerminate,
    LengthsNotMatch,
}

impl fmt::Display for BulkStringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BulkStringError::InvalidValue => {
                write!(f, "[BulkStringError] Invalid value.")
            }
            BulkStringError::InvalidFirstChar => {
                write!(f, "[BulkStringError] Invalid first char.")
            }
            BulkStringError::InvalidLength => {
                write!(f, "[BulkStringError] Invalid length.")
            }
            BulkStringError::InvalidLengthSeparator => {
                write!(f, "[BulkStringError] Invalid length separator.")
            }
            BulkStringError::InvalidTerminate => {
                write!(f, "[BulkStringError] Invalid terminate.")
            }
            BulkStringError::LengthsNotMatch => {
                write!(f, "[BulkStringError] Lengths do not match.")
            }
        }
    }
}

impl Error for BulkStringError {}

#[derive(Debug, Clone, PartialEq)]
pub struct BulkString(Bytes);

impl BulkString {
    pub fn new(input: &[u8]) -> Self {
        let length_string = input.len().to_string();
        let mut bytes = BytesMut::with_capacity(input.len() + length_string.len() + 5);
        bytes.put_u8(0x24); // "$"
        bytes.put_slice(length_string.as_bytes());
        bytes.put_u8(0x0d); // CR
        bytes.put_u8(0x0a); // LF
        bytes.put_slice(input);
        bytes.put_u8(0x0d); // CR
        bytes.put_u8(0x0a); // LF
        Self::from_bytes(bytes.freeze())
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

    pub fn while_valid(
        input: &[u8],
        start: &mut usize,
        end: &usize,
    ) -> Result<(), BulkStringError> {
        let mut index = *start;
        if index >= *end || input[index] != 0x24 {
            return Err(BulkStringError::InvalidFirstChar);
        }
        index += 1;
        if index + 1 >= *end
            || (input[index] == 0x30 && input[index + 1] >= 0x30 && input[index + 1] <= 0x39)
        {
            return Err(BulkStringError::InvalidLength);
        }
        while index < *end && input[index] >= 0x30 && input[index] <= 0x39 {
            index += 1;
        }
        if index + 1 >= *end || input[index] != 0x0d || input[index + 1] != 0x0a {
            return Err(BulkStringError::InvalidLengthSeparator);
        }
        let length = unsafe {
            String::from_utf8_unchecked(input[*start + 1..index].to_vec())
                .parse::<usize>()
                .unwrap()
        };
        index += 2;
        let value_start_index = index;
        while index < *end
            && index - value_start_index <= length
            && input[index] != 0x0d
            && input[index] != 0x0a
        {
            index += 1;
        }
        if length != index - value_start_index {
            return Err(BulkStringError::LengthsNotMatch);
        }
        if index + 1 >= *end || input[index] != 0x0d || input[index + 1] != 0x0a {
            return Err(BulkStringError::InvalidTerminate);
        }
        *start = index + 2;
        Ok(())
    }

    pub fn parse(input: &[u8], start: &mut usize, end: &usize) -> Result<Self, BulkStringError> {
        let mut index = *start;
        Self::while_valid(input, &mut index, end)?;
        let value = Self::from_slice(&input[*start..index]);
        *start = index;
        Ok(value)
    }
}

#[cfg(test)]
mod tests_bulk_string {
    use crate::BulkString;

    #[test]
    fn test_parse() {
        let string = "$6\r\nfoobar\r\n:100\r\n";
        let mut cursor = 0;
        assert_eq!(
            BulkString::parse(string.as_bytes(), &mut cursor, &string.len()).unwrap(),
            BulkString::new(b"foobar")
        );
        assert_eq!(cursor, 12);
    }
}
