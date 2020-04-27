pub use hello_world::emitent_service_server::{EmitentService, EmitentServiceServer};
use hello_world::{EmitentServiceReply, EmitentServiceRequest};
use tonic::{Code, Request, Response, Status};

use crate::emitent::EmitentRepository;

pub mod hello_world {
    tonic::include_proto!("emitent"); // The string specified here must match the proto package name
}

pub struct MyEmitentService<T: EmitentRepository> {
    repository: T,
}

impl<T> MyEmitentService<T>
where
    T: EmitentRepository,
{
    pub fn new(repository: T) -> Self {
        MyEmitentService { repository }
    }
}

#[tonic::async_trait]
impl<T: EmitentRepository> EmitentService for MyEmitentService<T> {
    async fn new_emitent(
        &self,
        request: Request<EmitentServiceRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<EmitentServiceReply>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);
        let moex = crate::emitent::Emitent::new(
            122,
            &request.get_ref().name,
            &request.get_ref().description,
        );
        match self.repository.store(&moex) {
            Ok(_) => {
                let reply = hello_world::EmitentServiceReply {
                    message: format!("Hello {}!", request.into_inner().name).into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
                };
                Ok(Response::new(reply)) // Send back our formatted greeting
            }
            Err(e) => Err(Status::new(Code::Internal, e.to_string())),
        }
    }
}
