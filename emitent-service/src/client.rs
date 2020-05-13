use hello_world::emitent_service_client::EmitentServiceClient;
use hello_world::EmitentServiceRequest;
use std::env;

pub mod hello_world {
    tonic::include_proto!("emitent");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uri = env::args().nth(1).unwrap();
    let mut client = EmitentServiceClient::connect(uri).await?;

    let request = tonic::Request::new(EmitentServiceRequest {
        name: "Sber".into(),
        description: "Sber bank".into(),
    });

    let response = client.new_emitent(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
