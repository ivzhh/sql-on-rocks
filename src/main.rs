#[macro_use]
extern crate serde_derive;

extern crate rmp_serde as rmps;
extern crate rocksdb;
extern crate serde;
extern crate serde_json;

use rmp_serialize::Encoder;
use rustc_serialize::Encodable;

use rocksdb::DB;
mod create_table;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    let mut db = DB::open_default("./rocks-sample").unwrap();

    db.put(b"123", b"456").unwrap();

    match db.get(b"123") {
        Ok(Some(value)) => println!("retrieved value {}", value.to_utf8().unwrap()),
        Ok(None) => println!("value not found"),
        Err(e) => println!("operational problem encountered: {}", e),
    }

    {
        // Convert the Point to a JSON string.
        let serialized = rmps::to_vec(&point).unwrap();

        // Prints serialized = {"x":1,"y":2}
        println!("serialized = {:?}", serialized);

        // Convert the JSON string back to a Point.
        let deserialized: Point = rmps::from_slice(&serialized[..]).unwrap();

        // Prints deserialized = Point { x: 1, y: 2 }
        println!("deserialized = {:?}", deserialized);
    }

    let val = (42u8, "the Answer");

    // The encoder borrows the bytearray buffer.
    let mut buf = [0u8; 13];

    // fixarray 	1001xxxx 	0x90 - 0x9f
    // the output is [92 2A AA 74 68 65 20 41 6E 73 77 65 72]
    // 92: array of 2 numbers
    // 2A: 42
    // AA: string of 10 chars
    // the rest: the string
    val.encode(&mut Encoder::new(&mut &mut buf[..]));

    println!("buf = {:?}", buf);
}
