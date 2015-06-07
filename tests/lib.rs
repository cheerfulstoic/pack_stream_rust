extern crate pack_stream;

use pack_stream::Value;

#[test]
fn it_unpacks_false() {
  let bytes = vec![0xC2u8];
  match pack_stream::unpack_stream(bytes)[0] {
  	Value::Boolean(b) => assert_eq!(false, b),
  	_ => panic!("It was not false")
  }
}

#[test]
fn it_unpacks_true() {
  let bytes = vec![0xC3u8];

  match pack_stream::unpack_stream(bytes)[0] {
  	Value::Boolean(b) => assert_eq!(true, b),
  	_ => panic!("It was not true")
  }
}

#[test]
fn it_unpacks_tiny_ints() {
	let mut bytes;

	bytes = vec![0x0u8];
	match pack_stream::unpack_stream(bytes)[0] {
		Value::TinyInt(i) => assert_eq!(0, i),
		_ => panic!("Value not TinyInt")
	}

	bytes = vec![0x0u8];
	match pack_stream::unpack_stream(bytes)[0] {
		Value::TinyInt(i) => assert_eq!(0, i),
		_ => panic!("Value not TinyInt")
	}

	bytes = vec![0x1u8];
	match pack_stream::unpack_stream(bytes)[0] {
		Value::TinyInt(i) => assert_eq!(1, i),
		_ => panic!("Value not TinyInt")
	}


	bytes = vec![0x7Fu8];
	match pack_stream::unpack_stream(bytes)[0] {
		Value::TinyInt(i) => assert_eq!(127, i),
		_ => panic!("Value not TinyInt")
	}
}

#[test]
fn it_unpacks_empty_tiny_text() {
  let bytes = vec![0x80];
  for i in pack_stream::unpack_stream(bytes) {
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
fn it_unpacks_populated_tiny_text() {
  let bytes = vec![0x85, 0x48, 0x65, 0x6C, 0x6C, 0x6F];
  for i in pack_stream::unpack_stream(bytes) {
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
fn it_unpacks_multiple_tiny_text_objects() {
	let bytes = vec![0x85, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x85, 0x48, 0x66, 0x6C, 0x6C, 0x70];
	for i in pack_stream::unpack_stream(bytes) {
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