use crate::errors::Result;
use oximod::set_global_client;
use std::env;

pub async fn init_db() -> Result<()> {
    let database_url = env::var("DATABASE_URL")?;

    set_global_client(database_url).await?;
    Ok(())
}

pub fn fetch_expose_url() -> Result<String> {
    let host: String =  std::env::var("HOST").unwrap_or("localhost".to_string());
    let port =  std::env::var("PORT").unwrap_or("5000".to_string());
    Ok(format!("{host}:{port}"))
}
