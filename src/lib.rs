extern crate rustc_serialize;

pub enum Value {
	Boolean(bool),
	TinyInt(u8),
	TinyText(Result<String, UnpackError>)
}

#[derive(Debug)]
pub enum UnpackError { UnreadableBytes }

pub fn unpack(mut bytes: Vec<u8>) -> Option<Value> {
	let header_byte = bytes.remove(0);

	match header_byte {
		//0xC0u8 => None,
		0xC2u8 => Some(Value::Boolean(false)),
		0xC3u8 => Some(Value::Boolean(true)),
		0u8...0x7Fu8 => Some(Value::TinyInt(header_byte)),
		0x80u8...0x8Fu8 => {
			let result = read_tiny_text(bytes);
			Some(Value::TinyText(result))
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