extern crate rustc_serialize;
extern crate byteorder;
// extern crate slice;
use byteorder::{BigEndian, ReadBytesExt, Error};

#[derive(Debug)]
pub enum Value {
	Boolean(bool),
	TinyInt(u8),
	TinyText(Result<String, UnpackError>),
	String(Result<String, UnpackError>),
	TinyList(Result<Vec<Value>, UnpackError>),
	List(Result<Vec<u8>, UnpackError>),
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
					0x00u8...0x7Fu8 => Value::TinyInt(header_byte),
					0x80u8...0x8Fu8 => self.unpack_tiny_text(header_byte),
					0x90u8...0x9Fu8 => self.unpack_tiny_list(header_byte),
					// 0xA0u8...0xAFu8 => {
						// self.unpack_tiny_map
					// },
					0xC1u8 => self.unpack_float64(),
					0xC2u8 => Value::Boolean(false),
					0xC3u8 => Value::Boolean(true),
					0xC8u8 => self.unpack_int8(),
					0xC9u8 => self.unpack_int16(),
					0xCAu8 => self.unpack_int32(),
					0xCBu8 => self.unpack_int64(),
					0xD0u8 => {
						let len = &self.content_len(1, "u8");
						self.unpack_string(len)
					},
					0xD1u8 => {
						let len = &self.content_len(3, "u16");
						self.unpack_string(len)
					},
					0xD3u8 => {
						let len = &self.content_len(7, "u32");
						self.unpack_string(len)
					},
					0xD4u8 => {
						let len = &self.content_len(1, "u8");
						self.unpack_list(len)
					},
					0xD5u8 => {
						let len = &self.content_len(3, "u16");
						self.unpack_list(len)
					},
					0xD6u8 => {
						let len = &self.content_len(7, "u32");
						self.unpack_list(len)
					},

					_ => Value::Unreadable
				}
			},
			_ => Value::Unreadable
		};
		self.buffer.push(packed_details);
		&self.buffer
	}

	fn unpack_tiny_list(&mut self, header_byte: u8) -> Value {
		let i = (header_byte - 0x90u8) as usize;
		let result = {
			let list_slice = Vec::from(&self.stream[0..i]);
			let mut slice_decoder = Decoder::new(list_slice);
			slice_decoder.unpack_all();
			slice_decoder.buffer
			// decoded
		};
		self.consume(i);
		Value::TinyList(Ok(result))
	}

	fn unpack_list(&mut self, list_length: &usize) -> Value {
		let result = {
			let slice = &self.stream[0..*list_length];
			Vec::from(slice)
		};
		&self.consume(*list_length);
		Value::List(Ok(result))
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

	// TODO: I think this needs to read the length value after marker
	fn unpack_string(&mut self, list_length: &usize) -> Value {
		let result = {
			let slice = &self.stream[0..*list_length];
			let vec = Vec::from(slice);
			match String::from_utf8(vec) {
				Ok(val) => Ok(val),
				_ => Err(UnpackError::UnreadableBytes)
			}
		};
		&self.consume(*list_length);
		Value::String(result)
	}

	fn content_len(&mut self, size: usize, read_as: &str) -> usize {
		let list_length_usize = {
			let mut list_length_slice: &[u8] = &self.stream[0..size];
			match read_as {
				"u8" => {
					let read_result: Result<u8, Error> = list_length_slice.read_u8();
					match read_result {
						Ok(val) => val as usize,
						Err(_) => 0
					}
				},
				"u16" => {
					let read_result: Result<u16, Error> = list_length_slice.read_u16::<BigEndian>();
					match read_result {
						Ok(val) => val as usize,
						Err(_) => 0
					}
				},
				"u32" => {
					let read_result: Result<u32, Error> = list_length_slice.read_u32::<BigEndian>();
					match read_result {
						Ok(val) => val as usize,
						Err(_) => 0
					}
				},
				_ => 0
			}
		};
		&self.consume(size);
		list_length_usize
	}
}

impl Iterator for Decoder {
	type Item = u8;
	fn next(&mut self) -> Option<u8> {
		Some(self.stream.remove(0))
	}
}
