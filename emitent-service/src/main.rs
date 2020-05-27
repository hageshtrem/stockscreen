use lapin::{
    options::*, publisher_confirm::Confirmation, types::FieldTable, BasicProperties, Connection,
    ConnectionProperties,
};
use log::info;
use std::env;
use std::sync::Arc;
use tonic::transport::Server;

mod emitent;
mod report;
mod repository;
mod server;
use server::EmitentServiceServer;
mod integration_events;
use integration_events::EventService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let addr = env::var("ADDR").expect("ADDR not setted").parse()?;
    let db_str = env::var("MONGO").expect("MONGO env not setted");
    let amqp_str = env::var("RABBIT").expect("RABBIT not setted");
    let storage = repository::Repository::new(&db_str)?;
    let es = EventService::new(&amqp_str).await?;
    let myservice = server::MyEmitentService::new(storage, es);

    Server::builder()
        .add_service(EmitentServiceServer::new(myservice))
        .serve(addr)
        .await?;

    Ok(())
}
