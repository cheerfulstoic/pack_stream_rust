extern crate pack_stream;

use pack_stream::Value;

#[test]
fn it_unpacks_false() {
  let bytes = vec![0xC2u8];

  match pack_stream::unpack(bytes).unwrap() {
    Value::Boolean(b) => assert_eq!(false, b),
    _ => panic!("Value not boolean"),
  }
}


#[test]
fn it_unpacks_true() {
  let bytes = vec![0xC3u8];

  match pack_stream::unpack(bytes).unwrap() {
    Value::Boolean(b) => assert_eq!(true, b),
    _ => panic!("Value not boolean"),
  }
}

#[test]
fn it_unpacks_tiny_ints() {
  let mut bytes;

  bytes = vec![0x0u8];
  match pack_stream::unpack(bytes).unwrap() {
    Value::TinyInt(i) => assert_eq!(0, i),
    _ => panic!("Value not boolean"),
  }

  bytes = vec![0x1u8];
  match pack_stream::unpack(bytes).unwrap() {
    Value::TinyInt(i) => assert_eq!(1, i),
    _ => panic!("Value not boolean"),
  }

  bytes = vec![0x7Fu8];
  match pack_stream::unpack(bytes).unwrap() {
    Value::TinyInt(i) => assert_eq!(127, i),
    _ => panic!("Value not boolean"),
  }

}
