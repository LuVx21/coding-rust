pub mod common;

use coding_common::strings;

#[test]
fn test_string_first_non() {
    common::setup();

    let test2 = &["", "", "rust", "lang"];
    println!("{:?}", strings::first_non_empty(test2));
    // 输出: Some("rust")

    // 使用方法4（宏）
    // println!("{:?}", first_non_empty1!("", "", "test"));
    // 输出: Some("test")
    // println!("{:?}", first_non_empty1!("", "", ""));
}
