use migration::{run};
#[tokio::main]
async fn main() {
    run().await.unwrap();
}
