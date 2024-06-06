#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   api::create().await.unwrap();
   Ok(())
}
