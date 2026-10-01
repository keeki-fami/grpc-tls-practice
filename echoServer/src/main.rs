use tonic::{transport::Server, Request, Response, Status};

use echo::echo_server::{Echo, EchoServer};
use echo::{EchoRequest, EchoResponse};

// echoを公開 → echo, echoserverなど
pub mod echo {
    tonic::include_proto!("echo");
}

#[derive(Debug, Default)]
pub struct MyEcho {}

#[tonic::async_trait]
impl Echo for MyEcho {
    async fn do_echo(
        &self,
        request: Request<EchoRequest>,
    ) -> Result<Response<EchoResponse>, Status> {
        println!("got a request: {:?}", request);

        let reply = EchoResponse {
            text: request.into_inner().text,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let echo = MyEcho::default();

    Server::builder()
        .add_service(EchoServer::new(echo))
        .serve(addr)
        .await?;

    Ok(())
}