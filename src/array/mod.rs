use crate::{BulkString, Error, Integer, RespError, RespType, SimpleString};
use bytes::{Buf, BufMut, Bytes, BytesMut};

pub const EMPTY_ARRAY: Array = Array(Bytes::from_static(b"*0\r\n"));
pub const NULL_ARRAY: Array = Array(Bytes::from_static(b"*-1\r\n"));

#[derive(Debug, Clone, PartialEq)]
pub struct Array(Bytes);

impl Array {
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn bytes(&self) -> Bytes {
        self.0.clone()
    }

    #[inline]
    pub fn to_vec(&self) -> Vec<u8> {
        let length = self.0.len();
        let mut vector = Vec::<u8>::with_capacity(length);
        unsafe {
            vector.set_len(length - 3);
        }
        self.bytes().copy_to_slice(vector.as_mut_slice());
        vector
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self == EMPTY_ARRAY
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        self == NULL_ARRAY
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
        if index + 3 >= *end {
            return Err(RespError::InvalidValue);
        }
        if input[index] != 0x2a {
            return Err(RespError::InvalidFirstChar);
        }
        index += 1;
        if input[index] == 0x2d {
            if input[index + 1] != 0x31
                || input[index + 2] != 0x0d
                || index + 3 == *end
                || input[index + 3] != 0x0a
            {
                return Err(RespError::InvalidNullValue);
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
        if length == 0 {
            *start = index;
            return Ok(());
        }
        if index >= *end {
            return Err(RespError::InvalidValue);
        }
        let mut count = 0;
        while count < length {
            match input[index] {
                0x2b => {
                    SimpleString::while_valid(input, &mut index, end)?;
                }
                0x2d => {
                    Error::while_valid(input, &mut index, end)?;
                }
                0x3a => {
                    Integer::while_valid(input, &mut index, end)?;
                }
                0x24 => {
                    BulkString::while_valid(input, &mut index, end)?;
                }
                0x2a => {
                    Self::while_valid(input, &mut index, end)?;
                }
                _ => {
                    return Err(RespError::InvalidValue);
                }
            }
            count += 1;
        }
        *start = index;
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

impl<'a> PartialEq<Array> for &'a Array {
    fn eq(&self, other: &Array) -> bool {
        self.0 == other.bytes()
    }
    fn ne(&self, other: &Array) -> bool {
        self.0 != other.bytes()
    }
}

pub struct ArrayBuilder {
    inner: Vec<RespType>,
}

impl ArrayBuilder {
    /// Builad a new Array Builder
    ///
    /// # Example
    /// ``` rust
    /// use resp_protocol::{Array, ArrayBuilder};
    ///
    /// let array_builder: ArrayBuilder = ArrayBuilder::new();
    /// let array: Array = array_builder.build();
    /// ```
    #[inline]
    pub fn new() -> ArrayBuilder {
        ArrayBuilder {
            inner: Vec::<RespType>::new(),
        }
    }

    #[inline]
    pub fn value(&mut self) -> Vec<RespType> {
        self.inner.clone()
    }

    /// Add a new value to Array Builder
    ///
    /// # Example
    /// ```rust
    /// use resp_protocol::{RespType, Array, ArrayBuilder, SimpleString};
    ///
    /// let mut array_builder: ArrayBuilder = ArrayBuilder::new();
    ///
    /// let simple_string: SimpleString = SimpleString::new(b"OK");
    ///
    /// array_builder.insert(RespType::SimpleString(simple_string));
    ///
    /// let array: Array = array_builder.build();
    /// ```
    #[inline]
    pub fn insert(&mut self, value: RespType) -> &mut Self {
        self.inner.push(value);
        self
    }

    #[inline]
    pub fn build(&self) -> Array {
        let length = self.inner.len();
        if length == 0 {
            return EMPTY_ARRAY;
        }
        let length_string = length.to_string();
        let mut total_bytes = length_string.len() + 3;
        for element in &self.inner {
            total_bytes += element.len();
        }
        let mut bytes = BytesMut::with_capacity(total_bytes);
        bytes.put_u8(0x2a); // "*"
        bytes.put_slice(length_string.as_bytes());
        bytes.put_u8(0x0d); // CR
        bytes.put_u8(0x0a); // LF
        for element in &self.inner {
            bytes.put(element.bytes());
        }
        Array(bytes.freeze())
    }
}

#[cfg(test)]
mod tests_array {
    use crate::{
        Array, ArrayBuilder, BulkString, Integer, RespType, SimpleString, EMPTY_ARRAY, NULL_ARRAY,
    };
    use bytes::Bytes;

    #[test]
    fn test_build_empty_array() {
        let array_builder = ArrayBuilder::new();
        assert_eq!(array_builder.build(), EMPTY_ARRAY)
    }

    #[test]
    fn test_build_array() {
        let mut array_builder = ArrayBuilder::new();
        array_builder.insert(RespType::SimpleString(SimpleString::new(b"foo")));
        assert_eq!(
            array_builder.build().bytes(),
            Bytes::from_static(b"*1\r\n+foo\r\n")
        );
        array_builder.insert(RespType::BulkString(BulkString::new(b"bar")));
        assert_eq!(
            array_builder.build().bytes(),
            Bytes::from_static(b"*2\r\n+foo\r\n$3\r\nbar\r\n")
        );
        array_builder.insert(RespType::Integer(Integer::new(-100)));
        assert_eq!(
            array_builder.build().bytes(),
            Bytes::from_static(b"*3\r\n+foo\r\n$3\r\nbar\r\n:-100\r\n")
        );
        let mut subarray_builder = ArrayBuilder::new();
        subarray_builder.insert(RespType::SimpleString(SimpleString::new(b"foo")));
        subarray_builder.insert(RespType::SimpleString(SimpleString::new(b"bar")));
        let subarray = subarray_builder.build();
        assert_eq!(
            subarray.bytes(),
            Bytes::from_static(b"*2\r\n+foo\r\n+bar\r\n")
        );
        array_builder.insert(RespType::Array(subarray));
        assert_eq!(
            array_builder.build().bytes(),
            Bytes::from_static(b"*4\r\n+foo\r\n$3\r\nbar\r\n:-100\r\n*2\r\n+foo\r\n+bar\r\n")
        );
    }

    #[test]
    fn test_parse_empty() {
        let string = "*0\r\n";
        let mut cursor = 0;
        let array = Array::parse(string.as_bytes(), &mut cursor, &string.len()).unwrap();

        assert_eq!(array, EMPTY_ARRAY);
        assert_eq!(cursor, 4);
    }

    #[test]
    fn test_parse_null() {
        let string = "*-1\r\n";
        let mut cursor = 0;
        let array = Array::parse(string.as_bytes(), &mut cursor, &string.len()).unwrap();

        assert_eq!(array, NULL_ARRAY);
        assert_eq!(cursor, 5);
    }

    #[test]
    fn parse_array_with_integers() {
        let string = "*3\r\n:1\r\n:2\r\n:3\r\n";
        let mut cursor = 0;
        let array = Array::parse(string.as_bytes(), &mut cursor, &string.len()).unwrap();

        let referance_array = ArrayBuilder::new()
            .insert(RespType::Integer(Integer::new(1)))
            .insert(RespType::Integer(Integer::new(2)))
            .insert(RespType::Integer(Integer::new(3)))
            .build();

        assert_eq!(array, referance_array);
        assert_eq!(cursor, 16);
    }

    #[test]
    fn parse_array_with_bulk_strings() {
        let string = "*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n";
        let mut cursor = 0;
        let array = Array::parse(string.as_bytes(), &mut cursor, &string.len()).unwrap();

        let referance_array = ArrayBuilder::new()
            .insert(RespType::BulkString(BulkString::new(b"foo")))
            .insert(RespType::BulkString(BulkString::new(b"bar")))
            .build();

        assert_eq!(array, referance_array);
        assert_eq!(cursor, 22);
    }
}
