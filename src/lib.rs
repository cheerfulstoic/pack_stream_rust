extern crate rustc_serialize;
extern crate byteorder;
// extern crate slice;
use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
pub enum Value {
	Boolean(bool),
	TinyInt(u8),
	TinyText(Result<String, UnpackError>),
	String(Result<String, UnpackError>),
	Int8(Result<i8, UnpackError>),
	Int16(Result<i16, UnpackError>),
	Int32(Result<i32, UnpackError>),
	Int64(Result<i64, UnpackError>),
	Float64(Result<f64, UnpackError>),
	Unreadable
}

#[derive(Debug)]
pub enum UnpackError { UnreadableBytes }

pub struct Decoder {
	pub stream: Vec<u8>,
	pub buffer: Vec<Value>,
}

impl Decoder {
	pub fn new(stream: Vec<u8>) -> Decoder {
		Decoder {
			stream: stream,
			buffer: vec![],
		}
	}

	fn consume(&mut self, i: usize) {
		for _ in 0..i { &self.next(); };
	}

	pub fn unpack_all(&mut self) -> &Vec<Value> {
		while self.stream.len() > 0 { self.unpack_next(); };
		&self.buffer
	}

	pub fn unpack_next(&mut self) -> &Vec<Value> {
		let header_option = self.next();
		let packed_details = match header_option {
			Some(header_byte) => {
				match header_byte {
					0u8...0x7Fu8 => Value::TinyInt(header_byte),
					0x80u8...0x8Fu8 => self.unpack_tiny_text(header_byte),
					0xC1u8 => self.unpack_float64(),
					0xC2u8 => Value::Boolean(false),
					0xC3u8 => Value::Boolean(true),
					0xC8u8 => self.unpack_int8(),
					0xC9u8 => self.unpack_int16(),
					0xCAu8 => self.unpack_int32(),
					0xCBu8 => self.unpack_int64(),
					0xD0u8...0xD2u8 => self.unpack_string(),
					_ => Value::Unreadable
				}
			},
			_ => Value::Unreadable
		};
		self.buffer.push(packed_details);
		&self.buffer
	}

	fn unpack_float64(&mut self) -> Value {
		let result = {
			let mut slice = &self.stream[0..8];
			&slice.read_f64::<BigEndian>().unwrap()
		};
		self.consume(8);
		Value::Float64(Ok(*result))
	}

	fn unpack_int8(&mut self) -> Value {
		let result = {
			let mut slice = &self.stream[0..1];
			&slice.read_i8().unwrap()
		};
		self.consume(1);
		Value::Int8(Ok(*result))
	}

	fn unpack_int16(&mut self) -> Value {
		let result = {
			let mut slice = &self.stream[0..2];
			&slice.read_i16::<BigEndian>().unwrap()
		};
		self.consume(2);
		Value::Int16(Ok(*result))
	}

	fn unpack_int32(&mut self) -> Value {
		let result = {
			let mut slice = &self.stream[0..4];
			&slice.read_i32::<BigEndian>().unwrap()
		};
		self.consume(4);
		Value::Int32(Ok(*result))
	}

	fn unpack_int64(&mut self) -> Value {
		let result = {
			let mut slice = &self.stream[0..8];
			&slice.read_i64::<BigEndian>().unwrap()
		};
		self.consume(8);
		Value::Int64(Ok(*result))
	}

	fn unpack_tiny_text(&mut self, header_byte: u8) -> Value {
		let i = (header_byte - 0x80u8) as usize;
		let result = {
			let bytes = &self.stream[0..i];
			let byte_vec = Vec::from(bytes);
			match String::from_utf8(byte_vec) {
				Ok(val) => Ok(val),
				_ => Err(UnpackError::UnreadableBytes)
			}
		};
		self.consume(i);
		Value::TinyText(result)
	}

	fn unpack_string(&mut self) -> Value {
		let i = self.next().unwrap() as usize;
		let result = {
			let bytes = &self.stream[0..i];
			let byte_vec = Vec::from(bytes);
			match String::from_utf8(byte_vec) {
				Ok(val) => Ok(val),
				_ => Err(UnpackError::UnreadableBytes)
			}
		};
		self.consume(i);
		Value::String(result)
	}
}

impl Iterator for Decoder {
	type Item = u8;
	fn next(&mut self) -> Option<u8> {
		Some(self.stream.remove(0))
	}
}
