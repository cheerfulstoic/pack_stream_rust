extern crate rustc_serialize;
extern crate byteorder;
// extern crate slice;
// use std::io::{Read, Bytes};
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
}

#[derive(Debug)]
pub enum UnpackError { UnreadableBytes }

pub fn unpack_stream(stream: Vec<u8>) -> Vec<Value> {
	let mut bytes_iter = stream.iter();
	let mut return_vec: Vec<Value> = vec![];
	let mut i: usize = 0;
	let mut movement: usize = 0;
	while let Some(byte) = bytes_iter.next() {
		let result = unpack(byte);
		match result {
			Some((unpacked_obj, len)) => {
				match len > 0 {
					true => {
						let mut content_slice = &stream[i + 1..(i + len)];
						movement = len;
						let to_return = match unpacked_obj {
											Value::TinyText(_val) => {
												let result = read_tiny_text(Vec::from(content_slice));
												Value::TinyText(result)
											},
											Value::String(_val) => {
												bytes_iter.next();
												let len = content_slice[0];
												i += 2;
												let content_slice = &stream[i..(i + len as usize)];
												movement = 1 + len as usize;
												Value::String(read_tiny_text(Vec::from(content_slice)))
											},
											Value::Float64(_val) => {
												let result = content_slice.read_f64::<BigEndian>().unwrap();
												Value::Float64(Ok(result))
											}
											Value::Int8(_val) => {
												let result = content_slice.read_i8().unwrap();
												Value::Int8(Ok(result))
											},
											Value::Int16(_val) => {
												let result = content_slice.read_i16::<BigEndian>().unwrap();
												Value::Int16(Ok(result))
											},
											Value::Int32(_val) => {
												let result = content_slice.read_i32::<BigEndian>().unwrap();
												Value::Int32(Ok(result))
											},
											Value::Int64(_val) => {
												let result = content_slice.read_i64::<BigEndian>().unwrap();
												Value::Int64(Ok(result))
											},
											_ => unpacked_obj
										};
						return_vec.push(to_return);
					},
					false => return_vec.push(unpacked_obj)
				};
				if bytes_iter.len() as usize > 0 {
					for _ in 0..movement - 1 { bytes_iter.next(); }
				}
				i = bytes_iter.len();
			},
			None => ()
		};
	};

	return_vec
}

pub fn unpack(header_byte: &u8) -> Option<(Value, usize)> {
	match *header_byte {
		//0xC0u8 => None,
		0xC1u8 => Some((Value::Float64(Err(UnpackError::UnreadableBytes)), 9)),

		0xC2u8 => Some((Value::Boolean(false), 0)),
		0xC3u8 => Some((Value::Boolean(true), 0)),

		0xC8u8 => Some((Value::Int8(Err(UnpackError::UnreadableBytes)), 2)),
		0xC9u8 => Some((Value::Int16(Err(UnpackError::UnreadableBytes)), 3)),
		0xCAu8 => Some((Value::Int32(Err(UnpackError::UnreadableBytes)), 5)),
		0xCBu8 => Some((Value::Int64(Err(UnpackError::UnreadableBytes)), 9)),

		0xD0u8...0xD2u8 => Some((Value::String(Err(UnpackError::UnreadableBytes)), 2)),
		// 0xD1u8 => Some((Value::String(Err(UnpackError::UnreadableBytes)), 2)),
		// 0xD2u8 => Some((Value::String(Err(UnpackError::UnreadableBytes)), 2)),

		// TinyInt
		0u8...0x7Fu8 => Some((Value::TinyInt(*header_byte), 0)),

		// TinyText
		0x80u8 => Some((Value::TinyText(Ok(String::new())), 0)),
		0x81u8...0x8Fu8 => {
			let len = bytes_ulimit(*header_byte, 0x80u8);
			Some((Value::TinyText(Err(UnpackError::UnreadableBytes)), len))
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
