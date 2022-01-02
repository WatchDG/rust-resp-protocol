use crate::RespError;
use bytes::{BufMut, Bytes, BytesMut};

pub const EMPTY_BULK_STRING: BulkString = BulkString(Bytes::from_static(b"$0\r\n\r\n"));
pub const NULL_BULK_STRING: BulkString = BulkString(Bytes::from_static(b"$-1\r\n"));

#[derive(Debug, Clone, PartialEq)]
pub struct BulkString(Bytes);

impl BulkString {
    /// Build a new Bulk String
    ///
    /// ``` rust
    /// use resp_protocol::BulkString;
    ///
    /// let bulk_string: BulkString = BulkString::new(b"foobar");
    /// println!("{:?}", bulk_string); // BulkString(b"$6\r\nfoobar\r\n")
    /// ```
    pub fn new(input: &[u8]) -> Self {
        let length = input.len();
        if length == 0 {
            return EMPTY_BULK_STRING;
        }
        let length_string = length.to_string();
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

    ///
    ///
    /// ``` rust
    /// use resp_protocol::{BulkString, EMPTY_BULK_STRING};
    ///
    /// let bulk_string: BulkString = EMPTY_BULK_STRING;
    /// println!("{:?}", bulk_string.is_empty()); // true
    ///
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self == EMPTY_BULK_STRING
    }

    ///
    ///
    /// ``` rust
    /// use resp_protocol::{BulkString, NULL_BULK_STRING};
    ///
    /// let bulk_string: BulkString = NULL_BULK_STRING;
    /// println!("{:?}", bulk_string.is_null()); // true
    ///
    /// ```
    #[inline]
    pub fn is_null(&self) -> bool {
        self == NULL_BULK_STRING
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

    pub fn while_valid(input: &[u8], start: &mut usize, end: &usize) -> Result<(), RespError> {
        let mut index = *start;
        if index >= *end || input[index] != 0x24 {
            return Err(RespError::InvalidFirstChar);
        }
        index += 1;

        if index + 3 >= *end {
            return Err(RespError::InvalidValue);
        }

        if input[index] == 0x2d {
            if input[index + 1] != 0x31 || input[index + 2] != 0x0d || input[index + 3] != 0x0a {
                return Err(RespError::InvalidValue);
            }
            *start = index + 4;
            return Ok(());
        }

        if input[index] == 0x30 && input[index + 1] >= 0x30 && input[index + 1] <= 0x39 {
            return Err(RespError::InvalidLength);
        }

        while index < *end && input[index] >= 0x30 && input[index] <= 0x39 {
            index += 1;
        }
        if index + 1 >= *end || input[index] != 0x0d || input[index + 1] != 0x0a {
            return Err(RespError::InvalidLengthSeparator);
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
            return Err(RespError::LengthsNotMatch);
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

impl<'a> PartialEq<BulkString> for &'a BulkString {
    fn eq(&self, other: &BulkString) -> bool {
        self.0 == other.bytes()
    }
    fn ne(&self, other: &BulkString) -> bool {
        self.0 != other.bytes()
    }
}

#[cfg(test)]
mod tests_bulk_string {
    use crate::{BulkString, EMPTY_BULK_STRING, NULL_BULK_STRING};
    use bytes::Bytes;

    #[test]
    fn test_new() {
        let bulk_string: BulkString = BulkString::new(b"foobar");
        assert_eq!(bulk_string.bytes(), Bytes::from_static(b"$6\r\nfoobar\r\n"));
    }

    #[test]
    fn test_new_empty() {
        let bulk_string: BulkString = BulkString::new(b"");
        assert_eq!(bulk_string.bytes(), Bytes::from_static(b"$0\r\n\r\n"));
    }

    #[test]
    fn test_from_bytes() {
        let bulk_string: BulkString =
            BulkString::from_bytes(Bytes::from_static(b"$6\r\nfoobar\r\n"));
        assert_eq!(bulk_string.bytes(), Bytes::from_static(b"$6\r\nfoobar\r\n"));
    }

    #[test]
    fn test_from_slice() {
        let bulk_string: BulkString =
            BulkString::from_slice(Vec::from("$6\r\nfoobar\r\n").as_slice());
        assert_eq!(bulk_string.bytes(), Bytes::from_static(b"$6\r\nfoobar\r\n"));
    }

    #[test]
    fn test_is_empty() {
        assert_eq!(EMPTY_BULK_STRING.is_empty(), true)
    }

    #[test]
    fn test_is_null() {
        assert_eq!(NULL_BULK_STRING.is_null(), true)
    }

    #[test]
    fn test_parse() {
        let string = "$6\r\nfoobar\r\n";
        let mut cursor = 0;
        assert_eq!(
            BulkString::parse(string.as_bytes(), &mut cursor, &string.len()).unwrap(),
            BulkString::new(b"foobar")
        );
        assert_eq!(cursor, 12);
    }

    #[test]
    fn test_parse_empty() {
        let string = "$0\r\n\r\n";
        let mut cursor = 0;
        assert_eq!(
            BulkString::parse(string.as_bytes(), &mut cursor, &string.len()).unwrap(),
            EMPTY_BULK_STRING
        );
        assert_eq!(cursor, 6);
    }

    #[test]
    fn test_parse_null() {
        let string = "$-1\r\n";
        let mut cursor = 0;
        assert_eq!(
            BulkString::parse(string.as_bytes(), &mut cursor, &string.len()).unwrap(),
            NULL_BULK_STRING
        );
        assert_eq!(cursor, 5);
    }
}
