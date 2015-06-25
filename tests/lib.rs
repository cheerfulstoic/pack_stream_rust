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
            Value::TinyText(val) => assert_eq!("", val),
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
            Value::TinyText(val) => assert_eq!("Hello", val),
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
            Value::TinyText(val) => { if val == "Hello" || val == "Hfllp" { () }; },
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
            Value::Int16(val) => assert_eq!(42, val),
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
            Value::Int32(val) => assert_eq!(42, val),
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
            Value::Int64(val) => assert_eq!(42, val),
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
            Value::Float64(val) => assert_eq!(1.1, val),
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
            Value::Float64(val) => assert_eq!(-1.1, val),
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
            Value::String(val) => assert_eq!("abcdefghijklmonpqrstuvwxyz", val),
            _ => panic!("Was not TT")
        }
    };

    let bytes = vec![0xD0, 0x18, 0x45, 0x6E, 0x20, 0xC3, 0xA5, 0x20, 0x66, 0x6C, 0xC3, 0xB6,
    0x74, 0x20, 0xC3, 0xB6, 0x76, 0x65, 0x72, 0x20, 0xC3, 0xA4, 0x6E, 0x67, 0x65, 0x6E];

    decoder = Decoder::new(bytes);
    decoder.unpack_all();
    for i in decoder.buffer {
        match i {
            Value::String(val) => assert_eq!("En å flöt över ängen", val),
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
                match content.len() == 0 {
                    true => (),
                    false => panic!("It contained... {}", content.len())
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
                match content.len() == 3 {
                    true => {
                        let mut i = 1;
                        for subval in content {
                            match subval {
                                Value::TinyInt(n) => assert_eq!(i, n),
                                _ => panic!("Was not a TinyInt")
                            };
                            i += 1;
                        }
                    },
                    false => panic!("It contained... {}", content.len())
                }
            },
            _ => panic!("Was not list")
        }
    }
}

#[test]
fn struct_unpacks_lists() {
    let bytes = vec![0xD4, 0x14, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x00];
    let mut decoder = Decoder::new(bytes);
    decoder.unpack_all();
    let mut list_content: Vec<u8> = vec![];
    for subvec in decoder.buffer {
        match subvec {
            Value::List(el) => {
                for num in el {
                    match num {
                        Value::TinyInt(val) => list_content.push(val),
                        _ => panic!("List did not contain int")
                    };
                };
            },
            _ => panic!("Was not a List")
        }
    };
    assert_eq!(vec![1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0], list_content)
}

#[test]
fn struct_unpacks_tiny_maps() {
    let bytes = vec![0xA1u8, 0x81u8, 0x61u8, 0x01u8];
    let mut decoder = Decoder::new(bytes);
    decoder.unpack_all();
    for i in decoder.buffer {
        match i {
            Value::TinyMap(tuples_vec) => {
                for pair in tuples_vec {
                    let (key, val) = pair;

                    match key {
                        Value::TinyText(k) => assert_eq!("a", k),
                        _ => println!("Key was not a string")
                    };
                    match val {
                        Value::TinyInt(v) => assert_eq!(1, v),
                        _ => println!("Val was not int")
                    }
                };
            },
            _ => panic!("Was not a tiny map")
        }
    }
}

#[test]
fn struct_unpacks_maps() {
    let bytes= vec![0xD8u8, 0x10u8, 0x81u8, 0x61u8, 0x01u8, 0x81u8, 0x62u8, 0x01u8, 0x81u8, 0x63u8, 0x03u8, 0x81u8, 0x64u8, 0x04u8, 0x81u8, 0x65u8, 0x05u8, 0x81u8, 0x66u8, 0x06u8, 0x81u8, 0x67u8, 0x07u8, 0x81u8, 0x68u8, 0x08u8, 0x81u8, 0x69u8, 0x09u8, 0x81u8, 0x6Au8, 0x00u8, 0x81u8, 0x6Bu8, 0x01u8, 0x81u8, 0x6Cu8, 0x02u8, 0x81u8, 0x6Du8, 0x03u8, 0x81u8, 0x6Eu8, 0x04u8, 0x81u8, 0x6Fu8, 0x05u8, 0x81u8, 0x70u8, 0x06u8];
    let keys = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p"];
    let values = vec![1,1,3,4,5,6,7,8,9,0,1,2,3,4,5,6];
    assert_eq!(keys.len(), values.len());

    let mut decoder = Decoder::new(bytes);
    decoder.unpack_all();
    for i in decoder.buffer {
        match i {
            Value::Map(tuples) => {
                let mut index = 0;

                for pair in tuples {
                    let (key, val) = pair;

                    match key {
                        Value::TinyText(name) => assert_eq!(keys[index], name),
                        _ => println!("Key was not a string")
                    };

                    match val {
                        Value::TinyInt(value) => assert_eq!(values[index], value),
                        _ => println!("Val was not int")
                    };

                    index = index + 1;
                }
            },
            _ => panic!("Was not a tiny map")
        }
    }
}

#[test]
fn struct_unpacks_tiny_struct() {
    let bytes = vec![0xB3u8, 0x01u8, 0x01u8, 0x02u8, 0x03u8];
    let mut decoder = Decoder::new(bytes);
    decoder.unpack_all();
    assert!(decoder.buffer.len() > 0);
    for i in decoder.buffer {
        match i {
            Value::Struct(s) => assert!(s.sig == 0x01u8),
            _ => panic!("Was not tiny struct! {:?}", i)
        }
    }
}

#[test]
fn struct_unpacks_normal_struct() {
    let bytes = vec![0xDCu8, 0x10u8, 0x01u8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8,
    0x08u8, 0x09u8, 0x00u8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8];
    let mut decoder = Decoder::new(bytes);
    decoder.unpack_all();
    for i in decoder.buffer {
        match i {
            Value::Struct(s) => assert!(s.sig == 0x01u8),
            _ => panic!("Was not a struct!")
        }
    }
    // We'll come back to this. It's nice to know it doesn't crash.
}

