use std::env;
use tonic::transport::Server;

mod emitent;
mod report;
mod repository;
mod server;
use server::EmitentServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = env::var("ADDR")?.parse()?;
    let conn_str = env::var("MONGO")?;
    let storage = repository::Repository::new(&conn_str)?;
    let myservice = server::MyEmitentService::new(storage);

    Server::builder()
        .add_service(EmitentServiceServer::new(myservice))
        .serve(addr)
        .await?;

    Ok(())
}
