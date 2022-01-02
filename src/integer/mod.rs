use crate::RespError;
use bytes::{Buf, BufMut, Bytes, BytesMut};

#[derive(Debug, Clone, PartialEq)]
pub struct Integer(Bytes);

impl Integer {
    #[inline]
    pub fn new(input: i64) -> Self {
        let string = input.to_string();
        let mut bytes = BytesMut::with_capacity(string.len() + 3);
        bytes.put_u8(0x3a); // ":"
        bytes.put_slice(string.as_bytes());
        bytes.put_u8(0x0d); // CR
        bytes.put_u8(0x0a); // LF
        Self::from_bytes(bytes.freeze())
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

    pub fn validate_value(input: &[u8]) -> Result<(), RespError> {
        let mut index = 0;
        let length = input.len();
        while index < length && input[index] != 0x0a && input[index] != 0x0d {
            index += 1;
        }
        if index != length {
            return Err(RespError::InvalidValue);
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

    pub fn while_valid(input: &[u8], start: &mut usize, end: &usize) -> Result<(), RespError> {
        let mut index = *start;
        if index >= *end || input[index] != 0x3a {
            return Err(RespError::InvalidFirstChar);
        }
        index += 1;
        while index < *end && input[index] != 0x0d && input[index] != 0x0a {
            index += 1;
        }
        if index + 1 >= *end || input[index] != 0x0d || input[index + 1] != 0x0a {
            return Err(RespError::InvalidTerminate);
        }
        *start = index + 2;
        Ok(())
    }

    pub fn parse(input: &[u8], start: &mut usize, end: &usize) -> Result<Self, RespError> {
        let mut index = *start;
        Self::while_valid(input, &mut index, end)?;
        let value = Self::from_slice(&input[*start..index]);
        *start = index;
        Ok(value)
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
    #[should_panic(expected = "InvalidValue")]
    fn test_validate_invalid_value() {
        let value = b"100\r\n";
        assert_eq!(Integer::validate_value(value).unwrap(), ())
    }

    #[test]
    fn test_parse() {
        let string = ":100\r\n+bar\r\n";
        let mut cursor = 0;
        let end = string.len();
        assert_eq!(
            Integer::parse(string.as_bytes(), &mut cursor, &end).unwrap(),
            Integer::new(100)
        );
        assert_eq!(cursor, 6);
    }
}
