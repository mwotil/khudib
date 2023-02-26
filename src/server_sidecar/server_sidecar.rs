#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::Mutex;
//use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};

// Import the protobuf stuff here
use khudib::khudib_proxy_server::{KhudibProxy, KhudibProxyServer};
use khudib::{KProxyRequest, KProxyResponse};
//use std::env;
use std::{net::SocketAddr, str::FromStr};

pub mod khudib {
    tonic::include_proto!("khudib");
}

#[derive(Debug, Default)]
pub struct KhudibProxyService {
    kbprequests: Mutex<Vec<KProxyRequest>>
}

#[tonic::async_trait]
impl KhudibProxy for KhudibProxyService {
    
    async fn create_khudib_proxy_request(&self, request: Request<KProxyRequest>) -> Result<Response<KProxyResponse>, Status> {
        
        let payload = request.into_inner();

        let kbprequest = KProxyRequest {
            client_id: payload.client_id,
            server_id: payload.server_id,
            request_id: payload.request_id,
            client_proxy_id: payload.client_proxy_id,
            server_proxy_id: payload.server_proxy_id,
            credit: payload.credit
        };

        self.kbprequests.lock().unwrap().push(kbprequest.clone());

        let message = KProxyResponse {
            kproxyresponse: Some(kbprequest)
        };

        Ok(Response::new(message))
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let khudib_proxy_service = KhudibProxyService::default();
    
    let mut address = String::new();
    address.push_str("127.0.0.1:5002");

    let add = SocketAddr::from_str(&address).unwrap();

    Server::builder()
        .add_service(KhudibProxyServer::new(khudib_proxy_service))
        .serve(add)
        .await?;

    Ok(())
}