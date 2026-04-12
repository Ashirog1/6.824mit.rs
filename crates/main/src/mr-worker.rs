#[tokio::main]
async fn main() -> anyhow::Result<()> {
    mr_core::worker::run(&mr_core::WordCount).await
}