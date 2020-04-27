use hello_world::emitent_service_client::EmitentServiceClient;
use hello_world::EmitentServiceRequest;

pub mod hello_world {
    tonic::include_proto!("emitent");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = EmitentServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(EmitentServiceRequest {
        name: "Sber".into(),
        description: "Sber bank".into(),
    });

    let response = client.new_emitent(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
