use std::error::Error;

use greeter_client::GreeterClient;

tonic::include_proto!("helloworld");

pub async fn query_hello() -> Result<(), Box<dyn Error>> {
    let mut client = GreeterClient::connect("http://127.0.0.1:8081").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Junhao".into(),
    });

    let response = client.say_hello(request).await?;

    println!("resposne from server: {:?}", response);

    Ok(())
}
