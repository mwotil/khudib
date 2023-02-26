#![allow(dead_code)]
#![allow(unused_variables)]

use khudib::khudib_proxy_client::KhudibProxyClient;
use khudib::{KProxyRequest};
use ulid_generator_rs::{ULIDGenerator, ULID};
//use std::env;


pub mod khudib {
    tonic::include_proto!("khudib");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut generator: ULIDGenerator = ULIDGenerator::new();
    let r_id: ULID = generator.generate().unwrap();

    let client_id = "127.0.0.1:5000".to_string();
    let client_proxy_id = "127.0.0.1:5001".to_string();
    let server_proxy_id = "127.0.0.1:5002".to_string();
    let server_id = "127.0.0.1:5003".to_string();

    let credit = 5;
    
    let req_id = String::new();


    let create_khudib_proxy_request = tonic::Request::new(KProxyRequest {
        client_id,
        server_id,
        request_id: r_id.to_string(),
        client_proxy_id,
        server_proxy_id,
        credit
    });

    let mut proxyclient = KhudibProxyClient::connect("http://127.0.0.1:5002").await?;
    let create_khudib_proxy_response = proxyclient.create_khudib_proxy_request(create_khudib_proxy_request).await?;

    println!("{:?}", create_khudib_proxy_response.into_inner().kproxyresponse);

    Ok(())
}
