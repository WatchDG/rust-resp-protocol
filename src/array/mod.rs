use crate::RespType;
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
    pub fn from_slice(input: &[u8]) -> Array {
        let bytes = Bytes::copy_from_slice(input);
        Array(bytes)
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

    #[inline]
    pub fn insert(&mut self, value: RespType) -> &Self {
        self.inner.push(value);
        self
    }

    #[inline]
    pub fn build(&self) -> Array {
        let length = self.inner.len();
        if length == 0 {
            EMPTY_ARRAY
        } else {
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
}

#[cfg(test)]
mod tests_array {
    use crate::{ArrayBuilder, BulkString, Error, Integer, RespType, SimpleString, EMPTY_ARRAY};
    use bytes::Bytes;

    #[test]
    fn build_empty_array() {
        let array_builder = ArrayBuilder::new();
        assert_eq!(array_builder.build(), EMPTY_ARRAY)
    }

    #[test]
    fn build_array_with_error() {
        let mut array_builder = ArrayBuilder::new();
        array_builder.insert(RespType::Error(Error::new(b"Invalid value.")));
        let array = array_builder.build();
        assert_eq!(
            array.bytes(),
            Bytes::from_static(b"*1\r\n-Invalid value.\r\n")
        )
    }

    #[test]
    fn build_array() {
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
}
