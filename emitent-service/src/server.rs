pub use hello_world::emitent_service_server::{EmitentService, EmitentServiceServer};
use hello_world::{
    list_emitents_reply::Emitent, ListEmitentsReply, NewEmitentReply, NewEmitentRequest,
};
use log::info;
use tonic::{Code, Request, Response, Status};

use crate::emitent::EmitentRepository;
use crate::integration_events::EventService;

pub mod hello_world {
    tonic::include_proto!("emitent"); // The string specified here must match the proto package name
}

pub struct MyEmitentService<T: EmitentRepository> {
    repository: T,
    brocker: EventService,
}

impl<T> MyEmitentService<T>
where
    T: EmitentRepository,
{
    pub fn new(repository: T, brocker: EventService) -> Self {
        MyEmitentService {
            repository,
            brocker,
        }
    }
}

#[tonic::async_trait]
impl<T: EmitentRepository> EmitentService for MyEmitentService<T> {
    async fn new_emitent(
        &self,
        request: Request<NewEmitentRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<NewEmitentReply>, Status> {
        // Return an instance of type HelloReply
        info!("Got a request: {:?}", request);
        let moex = crate::emitent::Emitent::new(
            122,
            &request.get_ref().name,
            &request.get_ref().description,
        );
        info!("store");
        if let Err(e) = self.repository.store(&moex) {
            return Err(Status::new(Code::Internal, e.to_string()));
        }
        // publish
        info!("publish");
        match self.brocker.publish(&moex).await {
            Ok(_) => {
                info!("publish success");
                let reply = hello_world::NewEmitentReply {
                    message: format!("Hello {}!", request.into_inner().name).into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
                };
                Ok(Response::new(reply))
            }
            Err(e) => Err(Status::new(Code::Internal, e.to_string())),
        }
    }

    async fn list_emitents(
        &self,
        request: Request<()>,
    ) -> Result<Response<ListEmitentsReply>, Status> {
        // TODO: logging decorator
        info!("Got a request: {:?}", request);
        match self.repository.get_all().await {
            Ok(emitents) => {
                let reply = emitents
                    .iter()
                    .map(|x| Emitent {
                        id: x.id,
                        name: x.name.clone(),
                        description: x.description.clone(),
                    })
                    .collect();
                Ok(Response::new(ListEmitentsReply { results: reply }))
            }
            Err(e) => Err(Status::new(Code::Internal, e.to_string())),
        }
    }
}
