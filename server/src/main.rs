use std::error::Error;

use hello::{greeter_server::GreeterServer, MyGreeter};
use tonic::transport::Server;

mod hello;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "0.0.0.0:8081".parse()?; 
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
