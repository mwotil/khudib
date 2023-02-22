#![allow(dead_code)]
#![allow(unused_variables)]

use breakwater::breakwater_client::BreakwaterClient;
use breakwater::{BreakwaterRequest};
use std::env;
//use std::collections::HashMap;

pub mod breakwater {
    tonic::include_proto!("breakwater");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = env::args().collect();
    if args.len() != 6 {
        println!("Usage: {} [ipv4 address] [port number] [Client ID] [No. of Requests] [No. of Credits]", args[0]);
        return Ok(())
    }

    let (ip_addr, port_no, id, requests, credits) = (&args[1], &args[2], &args[3], &args[4], &args[5]);

    let ip_addr = &args[1];
    let port_no = &args[2];
    let mut address = String::new();
    
    address.push_str("http://");
    address.push_str(ip_addr.as_str());
    address.push_str(":");
    address.push_str(port_no.as_str());

    let mut client = BreakwaterClient::connect(address).await?;

    let create_request = tonic::Request::new(BreakwaterRequest {
        id: id.parse::<i32>().unwrap(),
        requests: requests.parse::<i32>().unwrap(),
        credits: credits.parse::<i32>().unwrap()
    });

    let create_response = client.create_request(create_request).await?;

    println!("{:?}", create_response.into_inner().breakwater);

    Ok(())
}
