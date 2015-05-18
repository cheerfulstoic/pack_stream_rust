extern crate rustc_serialize;


//use serialize::{Encodable, Decodable};

pub fn unpack(bytes: Vec<u8>) -> bool {
  match bytes[0] {
    0xC2u8 => false,
    0xC3u8 => true,
    _ => panic!("Unknown header bytes: {}", bytes[0])
  }
}

