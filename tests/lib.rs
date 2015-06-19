extern crate pack_stream;

use pack_stream::{Value, Decoder};

#[test]
fn struct_unpacks_false() {
    let mut decoder = Decoder::new(vec![0xC2u8]);
    decoder.unpack_next();
    for i in decoder.buffer {
		match i {
			Value::Boolean(b) => assert_eq!(false, b),
			_ => panic!("It was not false")
		}
	}
}

#[test]
fn struct_unpacks_true() {
    let mut decoder = Decoder::new(vec![0xC3u8]);
    decoder.unpack_next();
    for i in decoder.buffer {
		match i {
			Value::Boolean(b) => assert_eq!(true, b),
			_ => panic!("It was not true")
		}
	}
}

#[test]
fn struct_unpacks_tiny_ints() {
	let mut decoder = Decoder::new(vec![0x0u8]);
	decoder.unpack_next();
	for i in decoder.buffer {
		match i {
			Value::TinyInt(i) => assert_eq!(0, i),
			_ => panic!("Value not TinyInt")
		}
	}

	decoder = Decoder::new(vec![0x1u8]);
	decoder.unpack_next();
	for i in decoder.buffer {
		match i {
			Value::TinyInt(i) => assert_eq!(1, i),
			_ => panic!("Value not TinyInt")
		}
	}

	decoder = Decoder::new(vec![0x7Fu8]);
	decoder.unpack_next();
	for i in decoder.buffer {
		match i {
			Value::TinyInt(i) => assert_eq!(127, i),
			_ => panic!("Value not TinyInt")
		}
	}
}

#[test]
fn struct_unpacks_empty_tiny_text() {
  let mut decoder = Decoder::new(vec![0x80]);
  decoder.unpack_next();
  for i in decoder.buffer {
  	match i {
	  	Value::TinyText(val) => {
	  		match val {
	  			Ok(v) => assert_eq!("", v),
	  			Err(_) => panic!("TT was not empty")
	  		}
	  	},
	  	_ => panic!("Was not TT")
  	}
  }
}

#[test]
fn struct_unpacks_populated_tiny_text() {
  let mut decoder = Decoder::new(vec![0x85, 0x48, 0x65, 0x6C, 0x6C, 0x6F]);
  decoder.unpack_next();
  for i in decoder.buffer {
  	match i {
	  	Value::TinyText(val) => {
	  		match val {
	  			Ok(v) => assert_eq!("Hello", v),
	  			Err(_) => panic!("TT was not empty")
	  		}
	  	},
	  	_ => panic!("Was not TT")
  	}
  }
}

#[test]
fn struct_unpacks_multiple_tiny_text_objects() {
	let mut decoder = Decoder::new(vec![0x85, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x85, 0x48, 0x66, 0x6C, 0x6C, 0x70]);
	decoder.unpack_all();
	for i in decoder.buffer {
	  	match i {
	  		Value::TinyText(val) => {
	  			match val {
	  				Ok(v) => {
	  					if v == "Hello" || v == "Hfllp" {
	  						()
	  					};
	  				},
	  				Err(_) => panic!("tt did not contain value")
	  			}
	  		},
		  	_ => panic!("Was not TT")
	  	}
	}
}

#[test]
fn struct_unpacks_i16() {
	let bytes = vec![0xC9, 0x00, 0x2A];
	let mut decoder = Decoder::new(bytes);
	decoder.unpack_next();
	for i in decoder.buffer {
		match i {
			Value::Int16(val) => {
				match val {
					Ok(v) => assert_eq!(42, v),
					Err(_) => panic!("Not 42")
				}
			},
			_ => panic!("Not i16")
		}
	}
}

#[test]
fn struct_unpacks_i32() {
	let bytes = vec![0xCA, 0x00, 0x00, 0x00, 0x2A];
	let mut decoder = Decoder::new(bytes);
	decoder.unpack_next();
	for i in decoder.buffer {
		match i {
			Value::Int32(val) => {
				match val {
					Ok(v) => assert_eq!(42, v),
					Err(_) => panic!("Not 42")
				}
			},
			_ => panic!("Not i32")
		}
	}
}

#[test]
fn struct_unpacks_i64() {
	let bytes = vec![0xCB, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2A];
	let mut decoder = Decoder::new(bytes);
	decoder.unpack_next();
	for i in decoder.buffer {
		match i {
			Value::Int64(val) => {
				match val {
					Ok(v) => assert_eq!(42, v),
					Err(e) => panic!("Not 42, {:?}", e)
				}
			},
			_ => panic!("Not i64")
		}
	}
}

#[test]
fn struct_unpacks_positive_f64() {
	let bytes = vec![0xC1, 0x3F, 0xF1, 0x99, 0x99, 0x99, 0x99, 0x99, 0x9A];
	let mut decoder = Decoder::new(bytes);
	decoder.unpack_next();
	for i in decoder.buffer {
		match i {
			Value::Float64(val) => {
				match val {
					Ok(v) => assert_eq!(1.1, v),
					Err(e) => panic!("Not 1.1, {:?}", e)
				}
			},
			_ => panic!("Not f64")
		}
	}
}

#[test]
fn struct_unpacks_negative_f64() {
	let bytes = vec![0xC1, 0xBF, 0xF1, 0x99, 0x99, 0x99, 0x99, 0x99, 0x9A];
	let mut decoder = Decoder::new(bytes);
	decoder.unpack_next();
	for i in decoder.buffer {
		match i {
			Value::Float64(val) => {
				match val {
					Ok(v) => assert_eq!(-1.1, v),
					Err(e) => panic!("Not 1.1, {:?}", e)
				}
			},
			_ => panic!("Not f64")
		}
	}
}

#[test]
fn struct_unpacks_strings() {
	let bytes = vec![0xD0, 0x1A, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6A,
						 0x6B, 0x6C, 0x6D, 0x6F, 0x6E, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76,
						 0x77, 0x78, 0x79, 0x7A];
	let mut decoder = Decoder::new(bytes);
	decoder.unpack_all();
    for i in decoder.buffer {
	  	match i {
		  	Value::String(val) => {
		  		match val {
		  			Ok(v) => assert_eq!("abcdefghijklmonpqrstuvwxyz", v),
		  			Err(_) => panic!("Did not match a thru z")
		  		}
		  	},
		  	_ => panic!("Was not TT")
	  	}
	};


	let bytes = vec![0xD0, 0x18, 0x45, 0x6E, 0x20, 0xC3, 0xA5, 0x20, 0x66, 0x6C, 0xC3, 0xB6,
					 0x74, 0x20, 0xC3, 0xB6, 0x76, 0x65, 0x72, 0x20, 0xC3, 0xA4, 0x6E, 0x67, 0x65, 0x6E];

	decoder = Decoder::new(bytes);
	decoder.unpack_all();
    for i in decoder.buffer {
	  	match i {
		  	Value::String(val) => {
		  		match val {
		  			Ok(v) => assert_eq!("En å flöt över ängen", v),
		  			Err(_) => panic!("Did not match Swedish phrase")
		  		}
		  	},
		  	_ => panic!("Was not string")
	  	}
	}
}

#[test]
fn struct_unpacks_tiny_list() {
	// TODO: This is not enough. This is a list of other packstream-encoded objects. It needs to return a vec of PackStream::Value enums.
	let bytes = vec![0x90u8];
	let mut decoder = Decoder::new(bytes);
	decoder.unpack_all();
	for i in decoder.buffer {
		match i {
			Value::TinyList(content) => {
				match content {
					Ok(val) => {
						match val.len() == 0 {
							true => (),
							false => panic!("It contained... {}", val.len())
						}
					},
					Err(e) => panic!("{:?}", e)
				}
			},
			_ => panic!("Was not list")
		}
	}


	let bytes = vec![0x93u8, 0x01u8, 0x02u8, 0x03u8];
	let mut decoder = Decoder::new(bytes);
	decoder.unpack_all();
	for i in decoder.buffer {
		match i {
			Value::TinyList(content) => {
				match content {
					Ok(val) => {
						match val.len() == 3 {
							true => {
								let mut i = 1;
								for subval in val {
									match subval {
										Value::TinyInt(n) => assert_eq!(i, n),
										_ => panic!("Was not a TinyInt")
									};
									i += 1;
								}
							},
							false => panic!("It contained... {}", val.len())
						}
					},
					Err(e) => panic!("{:?}", e)
				}
			},
			_ => panic!("Was not list")
		}
	}
}

#[test]
fn struct_unpacks_lists() {
	let bytes = vec![0xD4, 0x14, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x00];
	let len = &bytes.len();
	let mut decoder = Decoder::new(bytes);
	decoder.unpack_all();
	for i in decoder.buffer {
		match i {
			Value::List(content) => {
				match content {
					Ok(val) => {
						match val.len() == len - 2 {
							true => assert_eq!(vec![1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0], val),
							false => panic!("List contained {:?}", val)
						}
					},
					Err(e) => panic!("List err was {:?}", e)
				}
			},
			_ => panic!("Not list")
		}
	}
}