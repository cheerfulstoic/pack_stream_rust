extern crate rustc_serialize;
// extern crate slice;
use std::slice::{Iter};

#[derive(Debug)]
pub enum Value {
	Boolean(bool),
	TinyInt(u8),
	TinyText(Result<String, UnpackError>),
	Index(usize)
}

#[derive(Debug)]
pub enum UnpackError { UnreadableBytes }

pub fn unpack_stream(stream: Vec<u8>) -> Vec<Value> {
	let mut bytes_iter = stream.iter();
	let mut return_vec: Vec<Value> = vec![];
	let mut i: usize = 0;
	// let header_byte = bytes_iter.next();
	while let Some(byte) = bytes_iter.next() {
		let result = unpack(byte, &mut bytes_iter);
		match result {
			Some((good_value, thing)) => {
				match good_value {
					Value::Index(len) => {
						let content_slice = &stream[i + 1..(i + len)];
						// Consume n values
						for _step in 0..len - 1 { bytes_iter.next(); };
						i = bytes_iter.len();

						// Read the data
						let result = read_tiny_text(Vec::from(content_slice));
						return_vec.push(Value::TinyText(result))
					},
					_ => return_vec.push(good_value)
				}
			},
			None => ()
		}
	};

	return_vec
}

pub fn unpack(header_byte: &u8, _bytes_iter: &Iter<u8>) -> Option<(Value, usize)> {
	match *header_byte {
		//0xC0u8 => None,
		0xC2u8 => Some((Value::Boolean(false), 0)),
		0xC3u8 => Some((Value::Boolean(true), 0)),

		// TinyInt
		0u8...0x7Fu8 => Some((Value::TinyInt(*header_byte), 0)),

		// TinyText
		0x80u8 => Some((Value::TinyText(Ok(String::new())), 0)),
		0x81u8...0x8Fu8 => {
			let len = bytes_ulimit(*header_byte, 0x80u8);
			Some((Value::Index(len), len))
		},

		_ => None
	}
}

fn read_tiny_text(bytes: Vec<u8>) -> Result<String, UnpackError> {
	match String::from_utf8(bytes) {
		Ok(v) => Ok(v),
		_ => Err(UnpackError::UnreadableBytes)
	}
}

fn bytes_ulimit(header_byte: u8, offset: u8) -> usize {
	let bytes_to_read = header_byte - offset;
	(bytes_to_read + 1) as usize
}
