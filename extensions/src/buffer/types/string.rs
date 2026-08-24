use bytes::{Buf, BufMut};

use crate::buffer::BufferExt;

impl BufferExt for String {
  fn read(buf: &mut bytes::Bytes) -> Option<Self> {
    let len = buf.try_get_u16().ok()? as usize;
    if buf.remaining() < len {
      return None;
    }

    let bytes = buf.copy_to_bytes(len);
    let string = String::from_utf8(bytes.to_vec()).ok()?;

    Some(string)
  }

  fn write(&self, buf: &mut bytes::BytesMut) {
    let bytes = self.as_bytes();
    let len = bytes.len().min(u16::MAX as usize);
    buf.put_u16(len as u16);
    buf.put_slice(&bytes[..len]);
  }
}
