#![allow(dead_code)]
#![allow(unused_variables)]

use khudib::khudib_client::KhudibClient;
use khudib::{KRequest};
use std::env;
//use rand::{thread_rng, Rng};
//use unique_id::random::RandomGenerator;
use ulid_generator_rs::{ULIDGenerator, ULID};
use chrono::Utc;
use std::collections::HashMap;

pub mod khudib {
    tonic::include_proto!("khudib");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} [ipv4 address] [port number]", args[0]);
        return Ok(())
    }

    let mut monitor = HashMap::new();

    let (ip_addr, port_no) = (&args[1], &args[2]);

    let ip_addr = &args[1];
    let port_no = &args[2];
    let mut address = String::new();


    let mut server = String::new();
    server.push_str(&args[1]);
    server.push_str(":");
    server.push_str(&args[2]);
    
    address.push_str("http://");
    address.push_str(ip_addr.as_str());
    address.push_str(":");
    address.push_str(port_no.as_str());

    let client_id = "127.0.0.1:5555".to_string();
    
    let mut client = KhudibClient::connect(address).await?;

    
    //let mut rng = thread_rng();
    //let r_id: u32 = rng.gen();

    //let gen = RandomGenerator::default();
    //let r_id = gen.next_id();


    for _ in 0..10 {

        let mut generator: ULIDGenerator = ULIDGenerator::new();

        let mut timestamps = Vec::new();

        let r_id: ULID = generator.generate().unwrap();
        //let req_id = String::new();
        
        let create_khudib_request = tonic::Request::new(KRequest {
            client_id: client_id.clone(),
            server_id: server.clone(),
            request_id: r_id.to_string()
        });
        
        //println!("requestTimestamp: {}:", Utc::now().format("%S%.6f"));
        timestamps.push(Utc::now().format("%S%.6f"));
        
        let create_khudib_response = client.create_khudib_request(create_khudib_request).await?;
        
        //println!("responseTimestamp: {}:", Utc::now().format("%S%.6f"));
        timestamps.push(Utc::now().format("%S%.6f"));
        
        monitor.insert(r_id, timestamps);
        println!("{:?}", create_khudib_response.into_inner().khudib);
}

    for (key, value) in &monitor {
        println!("requestID: {} requestTimestamp: {} responseTimestamp {}", key, value[0], value[1]);
    }

    Ok(())
}
