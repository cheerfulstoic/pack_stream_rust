extern crate rustc_serialize;
// extern crate slice;
use std::slice::Iter;

#[derive(Debug)]
pub enum Value {
	Boolean(bool),
	TinyInt(u8),
	TinyText(Result<String, UnpackError>)
}

#[derive(Debug)]
pub enum UnpackError { UnreadableBytes }

pub fn unpack_stream(stream: Vec<u8>) -> Vec<Value> {
	let mut bytes_iter = stream.iter();
	let mut return_vec: Vec<Value> = vec![];
	// let header_byte = bytes_iter.next();
	while let Some(byte) = bytes_iter.next() {
		let result = unpack(byte, &mut bytes_iter);
		match result {
			Some(good_value) => return_vec.push(good_value),
			None => ()
		}
	};

	println!("return vec {:?}", return_vec);
	return_vec
}

pub fn unpack(header_byte: &u8, mut bytes_iter: &Iter<u8>) -> Option<Value> {
	// let &mut next_header = bytes_iter.next();
	match *header_byte {
		//0xC0u8 => None,
		0xC2u8 => Some(Value::Boolean(false)),
		0xC3u8 => Some(Value::Boolean(true)),

		// TinyInt
		0u8...0x7Fu8 => Some(Value::TinyInt(*header_byte)),

		// TinyText
		0x80u8 => Some(Value::TinyText(Ok(String::new()))),
		0x81u8...0x8Fu8 => {
			// let content_slice = &bytes[1..bytes_ulimit(header_byte, 0x80u8)];
			// let result = read_tiny_text(Vec::from(content_slice));
			// Some(Value::TinyText(result))
			Some(Value::Boolean(false))
		},

		_ => None
	}
	// Some(Value::Boolean(true))
}

// pub fn unpack(bytes: Vec<u8>) -> Option<Value> {
// 	let header_byte = bytes[0];

	// match header_byte {
	// 	//0xC0u8 => None,
	// 	0xC2u8 => Some(Value::Boolean(false)),
	// 	0xC3u8 => Some(Value::Boolean(true)),

	// 	// TinyInt
	// 	0u8...0x7Fu8 => Some(Value::TinyInt(header_byte)),

	// 	// TinyText
	// 	0x80u8 => Some(Value::TinyText(Ok(String::new()))),
	// 	0x81u8...0x8Fu8 => {
	// 		let content_slice = &bytes[1..bytes_ulimit(header_byte, 0x80u8)];
	// 		let result = read_tiny_text(Vec::from(content_slice));
	// 		Some(Value::TinyText(result))
	// 	},

	// 	_ => None
	// }
// }

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
