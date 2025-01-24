use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, from_document, Document},
    sync::{Client as SyncClient, Collection as SyncCollection},
    Client, Collection,
};
use serde::Deserialize;

const DSN: &str = "mongodb+srv://xxx:xxxx@xxxx";

#[derive(Debug, Deserialize)]
struct User {
    age: i32,
    #[serde(rename = "userName")]
    user_name: String,
}

#[test]
fn test_sync() -> mongodb::error::Result<()> {
    let client = SyncClient::with_uri_str(DSN)?;
    let database = client.database("boot");
    let coll: SyncCollection<Document> = database.collection("user");

    let result = coll.find_one(doc! { "userName": "foo_3" }).run()?;
    println!("查询结果:\n{:#?}", result.unwrap().get("_id"));

    let cursor = coll
        .find(doc! { "userName": {"$regex": "foo_3", "$options": "i"}})
        .sort(doc! { "_id": 1 })
        .run()?;
    for result in cursor {
        println!("{:?}", result?.get("userName"));
    }

    Ok(())
}

#[tokio::test]
async fn test_async() -> mongodb::error::Result<()> {
    let client = Client::with_uri_str(DSN).await?;
    let database = client.database("boot");
    let coll: Collection<Document> = database.collection("user");

    let result = coll.find_one(doc! { "userName": "foo_3" }).await?;
    println!("异步查询结果:\n{:#?}", result.unwrap().get("_id"));

    let mut cursor = coll
        .find(doc! { "userName": {"$regex": "foo_3", "$options": "i"}})
        .sort(doc! { "_id": 1 })
        .await?;
    while let Some(result) = cursor.next().await {
        match result {
            Ok(doc) => {
                // println!("异步游标: {:?}", doc.get("userName"));
                let user: User = from_document(doc)?;
                println!("User: {:?} {:?}", user.user_name, user.age);
            }
            Err(e) => {
                eprintln!("异步游标错误: {}", e);
            }
        }
    }

    Ok(())
}
