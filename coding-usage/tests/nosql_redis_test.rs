use r2d2::Pool;
use redis::AsyncCommands;
use redis::Commands;

const DSN: &str = "redis://xxx:xxx@xxx:xxx";

#[test]
fn do_something() -> redis::RedisResult<()> {
    let client = redis::Client::open(DSN)?;
    let mut con = client.get_connection()?;

    let rv: String = con.get("foo").unwrap();
    println!("{:?}", rv);

    Ok(())
}

#[test]
fn pool() -> redis::RedisResult<()> {
    let client = redis::Client::open(DSN).unwrap();
    let pool = Pool::builder().build(client).unwrap();
    let mut conn = pool.get().unwrap();

    // let _: () = conn.set("KEY", "VALUE").unwrap();
    let val: String = conn.get("foo").unwrap();
    println!("{:?}", val);

    Ok(())
}

#[test]
fn pipe() -> redis::RedisResult<()> {
    let mut con = redis::Client::open(DSN)?.get_connection()?;

    let (k1, k2): (i32, i32) = redis::pipe()
        .atomic()
        .set("key_1", 42)
        .ignore()
        .set("key_2", 43)
        .ignore()
        .get("key_1")
        .get("key_2")
        .query(&mut con)?;

    println!("{:?} {:?}", k1, k2);

    Ok(())
}

#[tokio::test]
async fn main() -> redis::RedisResult<()> {
    let client = redis::Client::open(DSN).unwrap();
    let mut con = client.get_multiplexed_async_connection().await?;

    let _: () = con.set("key1", b"foo").await?;
    redis::cmd("SET")
        .arg(&["key2", "bar"])
        .exec_async(&mut con)
        .await?;

    let result = redis::cmd("MGET")
        .arg(&["key1", "key2"])
        .query_async(&mut con)
        .await;
    assert_eq!(result, Ok(("foo".to_string(), b"bar".to_vec())));

    Ok(())
}
