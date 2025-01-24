use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct User {
    id: i64,
    user_name: String,
    age: u8,
}

#[sqlx::test]
async fn test_user_insert_and_query() -> sqlx::Result<()> {
    let pool = SqlitePool::connect("sqlite:/root/data/sqlite/main.db").await?;
    let rows = sqlx::query_as::<_, User>(
        r#"
        select id, user_name, age
        from user
        where id >= ?
        order by id
        "#,
    )
    .bind(1)
    .fetch_all(&pool)
    .await?;

    for row in rows {
        println!(
            "User: id={}, name={}, email={}",
            row.id, row.user_name, row.age
        );
    }

    Ok(())
}

#[test]
fn test_sqlite() {}
