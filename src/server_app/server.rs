#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};
use breakwater::breakwater_server::{BreakwaterServer, Breakwater};
use breakwater::{Bw, BreakwaterRequest, BreakwaterResponse};
use std::env;
use std::{net::SocketAddr, str::FromStr};

pub mod breakwater {
    tonic::include_proto!("breakwater");
}

#[derive(Debug, Default)]
pub struct BreakwaterService {
    bkwrequests: Mutex<Vec<Bw>>
}

#[tonic::async_trait]
impl Breakwater for BreakwaterService {
    async fn create_request(&self, request: Request<BreakwaterRequest>) -> Result<Response<BreakwaterResponse>, Status> {
        
        let payload = request.into_inner();

        let bkwrequest = Bw {
            id: payload.id,
            requests: payload.requests,
            credits: payload.credits
        };

        self.bkwrequests.lock().unwrap().push(bkwrequest.clone());

        let message = BreakwaterResponse {
            breakwater: Some(bkwrequest)
        };

        Ok(Response::new(message))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: {} [ipv4 address] [port number]", args[0]);
        return Ok(())
    }

    let ip_addr = &args[1];
    let port_no = &args[2];
    let mut address = String::new();
    
    address.push_str(ip_addr.as_str());
    address.push_str(":");
    address.push_str(port_no.as_str());

    let add = SocketAddr::from_str(&address).unwrap();

    let breakwater_service = BreakwaterService::default();

    Server::builder()
        .add_service(BreakwaterServer::new(breakwater_service))
        .serve(add)
        .await?;

    Ok(())
}
