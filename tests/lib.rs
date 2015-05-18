extern crate pack_stream;

#[test]
fn it_unpacks_false() {
  let bytes = vec![0xC2u8];

  assert_eq!(false, pack_stream::unpack(bytes));
}

#[test]
fn it_unpacks_true() {
  let bytes = vec![0xC3u8];

  assert_eq!(true, pack_stream::unpack(bytes));
}
