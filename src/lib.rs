extern crate rustc_serialize;
extern crate byteorder;
// extern crate slice;
use byteorder::{BigEndian, ReadBytesExt, Error};
use std::collections::{HashMap};

pub type PackStreamResult<T> = Result<T, UnpackError>;

#[derive(Debug)]
pub enum Value {
	Boolean(bool),
	TinyInt(u8),
	TinyText(String),
	String(String),
	TinyList(Vec<Value>),
	TinyMap(Vec<(Value, Value)>),
	List(Vec<Value>),
	Map(Vec<(Value, Value)>),
	Int8(i8),
	Int16(i16),
	Int32(i32),
	Int64(i64),
	Float64(f64),
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
					0x00u8...0x7Fu8 => Ok(Value::TinyInt(header_byte)),
					0x80u8...0x8Fu8 => self.unpack_tiny_text(header_byte),
					0x90u8...0x9Fu8 => self.unpack_tiny_list(header_byte),
					0xA0u8...0xAFu8 => self.unpack_tiny_map(header_byte),
					0xC1u8 => self.unpack_float64(),
					0xC2u8 => Ok(Value::Boolean(false)),
					0xC3u8 => Ok(Value::Boolean(true)),
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
					0xD8u8 => {
						let len = &self.content_len(1, "u8");
						self.unpack_map(len)
					},

					_ => Err(UnpackError::UnreadableBytes)
				}
			},
			_ => Err(UnpackError::UnreadableBytes)
		};
		// packed_details.wtffff();
		match packed_details {
			Ok(val) => self.buffer.push(val),
			_ => ()
		};
		// self.buffer.push(packed_details);
		&self.buffer
	}

	fn unpack_tiny_map(&mut self, header_byte: u8) -> PackStreamResult<Value> {
		let pairs = (header_byte - 0xA0u8) as usize;
		let result_tuples = self.map_population(&pairs);
		// self.consume(pairs);
		Ok(Value::TinyMap(result_tuples))
	}

	fn map_population(&mut self, map_length: &usize) -> Vec<(Value, Value)> {
		let mut result_tuples: Vec<(Value, Value)> = vec![];
		for _ in (0..*map_length) {
			let key = {
				self.unpack_next();
				self.buffer.pop().unwrap()
			};
			let value = {
				self.unpack_next();
				self.buffer.pop().unwrap()
			};
			result_tuples.push((key, value))
		};
		result_tuples
	}

	fn unpack_map(&mut self, map_length: &usize) -> PackStreamResult<Value> {
		let result_tuples = self.map_population(&map_length);
		Ok(Value::Map(result_tuples))
	}

	fn unpack_tiny_list(&mut self, header_byte: u8) -> PackStreamResult<Value> {
		let i = (header_byte - 0x90u8) as usize;
		let result = {
			let list_slice = Vec::from(&self.stream[0..i]);
			let mut slice_decoder = Decoder::new(list_slice);
			slice_decoder.unpack_all();
			slice_decoder.buffer
			// decoded
		};
		self.consume(i);
		Ok(Value::TinyList(result))
	}

	fn unpack_list(&mut self, list_length: &usize) -> PackStreamResult<Value> {
		let result = {
			let slice = &self.stream[0..*list_length];
			let mut slice_decoder = Decoder::new(Vec::from(slice));
			slice_decoder.unpack_all();
			slice_decoder.buffer
		};
		&self.consume(*list_length);
		Ok(Value::List(result))
	}

	fn unpack_float64(&mut self) -> PackStreamResult<Value> {
		let result = {
			let mut slice = &self.stream[0..8];
			&slice.read_f64::<BigEndian>().unwrap()
		};
		self.consume(8);
		Ok(Value::Float64(*result))
	}

	fn unpack_int8(&mut self) -> PackStreamResult<Value> {
		let result = {
			let mut slice = &self.stream[0..1];
			&slice.read_i8().unwrap()
		};
		self.consume(1);
		Ok(Value::Int8(*result))
	}

	fn unpack_int16(&mut self) -> PackStreamResult<Value> {
		let result = {
			let mut slice = &self.stream[0..2];
			&slice.read_i16::<BigEndian>().unwrap()
		};
		self.consume(2);
		Ok(Value::Int16(*result))
	}

	fn unpack_int32(&mut self) -> PackStreamResult<Value> {
		let result = {
			let mut slice = &self.stream[0..4];
			&slice.read_i32::<BigEndian>().unwrap()
		};
		self.consume(4);
		Ok(Value::Int32(*result))
	}

	fn unpack_int64(&mut self) -> PackStreamResult<Value> {
		let result = {
			let mut slice = &self.stream[0..8];
			&slice.read_i64::<BigEndian>().unwrap()
		};
		self.consume(8);
		Ok(Value::Int64(*result))
	}

	fn unpack_tiny_text(&mut self, header_byte: u8) -> PackStreamResult<Value> {
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
		Ok(Value::TinyText(result.unwrap()))
	}

	// TODO: I think this needs to read the length value after marker
	fn unpack_string(&mut self, list_length: &usize) -> PackStreamResult<Value> {
		let result = {
			let slice = &self.stream[0..*list_length];
			let vec = Vec::from(slice);
			match String::from_utf8(vec) {
				Ok(val) => Ok(val),
				_ => Err(UnpackError::UnreadableBytes)
			}
		};
		&self.consume(*list_length);
		Ok(Value::String(result.unwrap()))
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
