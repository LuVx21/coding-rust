use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const URL: &str = "https://api.myip.com";

#[derive(Debug, Deserialize)]
struct ApiRes {
    ip: String,
}

#[test]
fn test_m1() {
    let result: Result<(), Error> = tokio::runtime::Runtime::new()
    .unwrap()
    .block_on(async {
        let response = reqwest::get(URL)
            .await?;

        if response.status().is_success() {
            let post: ApiRes = response.json().await?;
            println!("ip地址: {}", post.ip);
        } else {
            println!("请求失败,状态: {}", response.status());
        }

        Ok(())
    });

    result.unwrap();
}


#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: Option<i32>,
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: i32,
}

#[tokio::test]
async fn post() -> Result<(), Error> {
    let new_post = Post {
        id: None,
        title: "Reqwest.rs".into(),
        body: "https://docs.rs/reqwest".into(),
        user_id: 1,
    };
    let new_post: Post = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&new_post)
        .send()
        .await?
        .json()
        .await?;

    println!("{new_post:#?}");
    Ok(())
}

#[tokio::test]
async fn test_map() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{resp:#?}");
    Ok(())
}