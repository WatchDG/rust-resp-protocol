use bytes::{BufMut, Bytes, BytesMut};

pub struct BulkString(Bytes);

impl BulkString {
    pub fn new(input: &[u8]) -> BulkString {
        let length_string = input.len().to_string();
        let mut bytes = BytesMut::with_capacity(input.len() + length_string.len() + 5);
        bytes.put_u8(0x24); // "$"
        bytes.put_slice(length_string.as_bytes());
        bytes.put_u8(0x0d); // CR
        bytes.put_u8(0x0a); // LF
        bytes.put_slice(input);
        bytes.put_u8(0x0d); // CR
        bytes.put_u8(0x0a); // LF
        BulkString(bytes.freeze())
    }

    #[inline]
    pub fn bytes(&self) -> Bytes {
        self.0.clone()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
