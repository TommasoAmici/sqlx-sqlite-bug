use sqlx::sqlite::SqlitePool;
use time::Date;

#[derive(Debug)]
pub enum Error {
    DBFailedToConnect,
    DBFailedQuery,
}

pub struct FilterQueryParams {
    pub project: Option<String>,
    pub branch: Option<String>,
    pub date_start: Option<Date>,
    pub date_end: Option<Date>,
    pub sensitivity: Option<i32>,
}

pub struct LanguageStream {
    pub language: String,
    pub date: Date,
    pub count: i32,
}

pub async fn languages_stream(
    pool: &SqlitePool,
    user_id: i64,
    params: &FilterQueryParams,
) -> Result<Vec<LanguageStream>, Error> {
    let mut conn = pool.acquire().await.map_err(|_| Error::DBFailedToConnect)?;
    let results = sqlx::query_file_as!(
        LanguageStream,
        // switch between these two to see the difference
        // "src/languages_stream_broken.sql",
        "src/languages_stream.sql",
        user_id,
        params.project,
        params.branch,
        params.date_start,
        params.date_end,
        params.sensitivity,
    )
    .fetch_all(&mut conn)
    .await
    .map_err(|_| Error::DBFailedQuery)?;

    Ok(results)
}

fn main() {
    println!("Hello World");
}
