use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
use http::{header, Method};
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    // CORS設定
    let cors_layer = CorsLayer::new()
        .allow_origin(AllowOrigin::list(vec!["localhost:3000".parse().unwrap()]))
        .allow_headers(AllowHeaders::list(vec![
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
        ]))
        .allow_methods(AllowMethods::list(vec![Method::GET, Method::POST]));

    Server::builder()
        .accept_http1(true)
        .layer(cors_layer) // CORS設定を適用
        .add_service(tonic_web::enable(GreeterServer::new(greeter)))
        .serve(addr)
        .await?;

    Ok(())
}
