use kodiak_sets::Sequence;
use sqlx::sqlite::{SqlitePoolOptions, SqlitePool};
use sqlx::Executor;

const DATABASE_URL: &str = "./test.db";
const N_ITERATIONS: usize = 100_000;

#[tokio::main]
async fn main() {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .after_connect(|conn, _meta| Box::pin(async move {
            conn.execute("PRAGMA journal_mode = WAL;").await;
            conn.execute("PRAGMA synchronous = NORMAL;").await;
            conn.execute("PRAGMA cache_size = 1000000;").await;
            conn.execute("PRAGMA locking_mode = NORMAL;").await;
            conn.execute("PRAGMA temp_store = MEMORY;").await;
            Ok(())}))
        .connect(DATABASE_URL)
        .await
        .unwrap();

    let mut seq: Sequence<String> = Sequence::new();
    seq.push("A".to_string());
    seq.push("B".to_string());
    seq.push("C".to_string());

    let mut n: usize = 0;
    // With 79_999_980 insertions, the two elements added last are not longer distinguishable
    while n < N_ITERATIONS {
        n += 1;
        seq.insert(n, "A".to_string());
    }

    n = 0;
    while n <= seq.len() {
            if let Some((numerator, denominator, payload)) = seq.non_iterating_next() {
            insert(&pool, numerator, denominator, payload).await.unwrap();
        };
        n += 1;
    }
}

async fn insert(pool: &SqlitePool, n: u64, d: u64, payload: &str) -> Result<(), sqlx::Error> {
    let numerator = n as i64;
    let denominator = d as i64;

    let result = sqlx::query!(
        r#"INSERT INTO sequence
                       (numerator,   denominator, payload)
                VALUES ($1,         $2,           $3);"#,
        numerator,
        denominator,
        payload
    )
    .execute(pool)
    .await?;

    Ok(())
}
