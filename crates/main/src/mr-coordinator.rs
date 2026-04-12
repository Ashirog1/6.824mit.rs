#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    anyhow::ensure!(!args.is_empty(), "Usage: mr-coordinator <files...>");
    mr_core::coordinator::run(args, 10).await
}
