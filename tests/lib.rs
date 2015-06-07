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

// #[test]
// fn it_unpacks_empty_tiny_text() {
//   let bytes = vec![0x80];

//   match pack_stream::unpack(bytes)[0] {
//     Value::TinyText(i) => {
//       match i {
//         Ok(v) => assert_eq!("", v),
//         Err(_e) => panic!("TinyText was not empty!"),
//       }
//     },
//     _ => panic!("Value not TinyText"),
//   }
// }

// #[test]
// fn it_unpacks_populated_tiny_text() {
//   let bytes = vec![0x85, 0x48, 0x65, 0x6C, 0x6C, 0x6F];

//   match pack_stream::unpack(bytes).unwrap() {
//     Value::TinyText(i) => {
//       match i {
//         Ok(v) => assert_eq!("Hello", v),
//         Err(_e) => panic!("TinyText was empty!"),
//       }
//     },
//     _ => panic!("Value not TinyText"),
//   }
// }

