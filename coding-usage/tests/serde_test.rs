#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    #[serde(rename = "userId")]
    user_id: i32,
    name: String,
    age: u8,
}
#[derive(Debug, Deserialize)]
struct Address {
    city: String,
    zip: String,
}

#[test]
fn test_m1() {
    let p = User {
        id: None,
        user_id: 101,
        name: "姓名".into(),
        age: 18,
    };

    let json = serde_json::to_string(&p).unwrap();
    println!("序列化结果: {}", json);

    let o: User = serde_json::from_str(&json).unwrap();
    println!("反序列化结果: {:?}", o);
}

#[test]
fn test_m2() {
    let json = r#"
        {
            "name": "张三",
            "age": 30,
            "is_student": false,
            "scores": [95, 87, 92],
            "address": {
                "city": "上海",
                "zip": "12345"
            }
        }
    "#;

    let map: HashMap<String, Value> = serde_json::from_str(json).unwrap();

    if let Some(v) = map.get("address") {
        println!("map值: {}", v);

        let a1: Address = serde_json::from_value(v.clone()).unwrap();
        println!("反序列化为Address: {:?}", a1);

        let a2: HashMap<String, Value> = serde_json::from_value(v.clone()).unwrap();
        println!("反序列化为Map: {:?}", a2);
    }
}
