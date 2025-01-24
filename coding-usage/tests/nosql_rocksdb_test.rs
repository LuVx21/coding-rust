// use rocksdb::{Options, DB};

// // apt install -y llvm clang libclang-dev

// const PATH: &str = "/root/data/rocksdb_rust1";

// #[test]
// fn m1() {
//     {
//         let db = DB::open_default(PATH.to_owned() + "_tmp").unwrap();

//         db.put(b"foo", b"bar").unwrap();

//         match db.get(b"foo") {
//             Ok(Some(value)) => println!("值: {}", String::from_utf8(value).unwrap()),
//             Ok(None) => println!("无值"),
//             Err(e) => println!("错误: {}", e),
//         }

//         db.delete(b"foo").unwrap();
//     }
//     let _ = DB::destroy(&Options::default(), PATH.to_owned() + "_tmp");
// }

// #[test]
// fn test_m1() -> Result<(), Box<dyn std::error::Error>> {
//     let mut opts = Options::default();
//     opts.create_if_missing(true);
//     let db = DB::open(&opts, PATH)?;

//     db.put(b"foo", b"bar")?;

//     match db.get(b"foo") {
//         Ok(Some(value)) => println!("值: {}", String::from_utf8_lossy(&value)),
//         Ok(None) => println!("无值"),
//         Err(e) => println!("错误: {}", e),
//     }

//     db.delete(b"foo")?;

//     Ok(())
// }
