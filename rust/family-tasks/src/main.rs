use family_tasks::{proto, task_adapters};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_endpoint = std::env::var("REBOOT_RUST_DATABASE_ENDPOINT").map_err(
        |_| "REBOOT_RUST_DATABASE_ENDPOINT must include a scheme, e.g. http://127.0.0.1:50053",
    )?;
    let listen_addr = std::env::var("REBOOT_RUST_LISTEN_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:50051".to_owned())
        .parse()?;
    let (writes, reads) = task_adapters(&database_endpoint).await?;

    tonic::transport::Server::builder()
        .add_service(proto::task_writes_server::TaskWritesServer::new(writes))
        .add_service(proto::task_reads_server::TaskReadsServer::new(reads))
        .serve(listen_addr)
        .await?;
    Ok(())
}
