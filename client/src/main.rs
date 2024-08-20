use std::error::Error;

use hello::query_hello;

mod hello;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    query_hello().await?;
    Ok(())
}
