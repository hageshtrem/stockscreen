use tonic::transport::Server;

mod emitent;
mod report;
mod repository;
mod server;
use server::EmitentServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get from environment
    let storage = repository::Repository::new("mongodb://localhost:27017")?;

    let myservice = server::MyEmitentService::new(storage);
    // get from environment
    let addr = "[::1]:50051".parse()?;

    Server::builder()
        .add_service(EmitentServiceServer::new(myservice))
        .serve(addr)
        .await?;

    Ok(())
}
