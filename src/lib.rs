extern crate rustc_serialize;

pub enum Value {
  Boolean(bool),
  TinyInt(u8),
}

pub fn unpack(bytes: Vec<u8>) -> Option<Value> {
  let header_byte = bytes[0];

  match header_byte {
    //0xC0u8 => None,
    0xC2u8 => Some(Value::Boolean(false)),
    0xC3u8 => Some(Value::Boolean(true)),
    0u8...0x7Fu8 => Some(Value::TinyInt(header_byte)),
    _ => None
  }
}

