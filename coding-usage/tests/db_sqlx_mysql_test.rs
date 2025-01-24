use sqlx::mysql::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct User {
    id: i64,
    user_name: String,
    password: String,
    age: Option<i32>,
}

const URI: &str = "mysql://root:xxx@luvx.rx:3306/boot?charset=utf8mb4";

#[sqlx::test]
async fn test_user_insert_and_query() -> sqlx::Result<()> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(URI)
        .await?;

    let rows = sqlx::query!(
        r#"
        select id, user_name, password, age
        from user
        where id >= ?
        order by id
        "#,
        1
    )
    .fetch_all(&pool)
    .await?;

    for row in rows {
        println!(
            "- [{}] {}: {} {}",
            row.id, &row.user_name, &row.password, row.age.unwrap_or(-1),
        );
    }

    let users = sqlx::query_as!(
        User,
        r#"
        select id, user_name, password, age
        from user
        where id >= ?
        order by id
        "#,
        1
    )
    .fetch_all(&pool)
    .await?;

    for row in users {
        println!(
            "User: id={}, name={}, password={} age={}",
            row.id,
            row.user_name,
            row.password,
            row.age.unwrap_or(-1)
        );
    }

    Ok(())
}

#[sqlx::test]
async fn test_transaction() -> sqlx::Result<()> {
    let pool = MySqlPool::connect(URI).await?;

    let mut tx = pool.begin().await?;

    sqlx::query!(
        r#"
        insert into user (user_name, password, age)
        values (?, ?, ?)
        "#,
        "姓名",
        "密码",
        18
    )
    .execute(&mut *tx)
    .await?;

    let user = sqlx::query_as!(
        User,
        r#"
        select id, user_name, password, age
        from user
        where user_name = ?
        "#,
        "姓名"
    )
    .fetch_one(&mut *tx)
    .await?;

    assert_eq!(user.user_name, "姓名");

    // 回滚事务，确保测试不会修改数据库
    tx.rollback().await?;

    Ok(())
}
