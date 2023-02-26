#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::Mutex;
//use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};
use khudib::khudib_server::{KhudibServer, Khudib};
use khudib::{Mkhudib, KRequest, KResponse};
use std::env;
use std::{net::SocketAddr, str::FromStr};
use chrono::Utc;
use std::{thread, time};


pub mod khudib {
    tonic::include_proto!("khudib");
}

#[derive(Debug, Default)]
pub struct KhudibService {
    kbrequests: Mutex<Vec<Mkhudib>>
}



#[tonic::async_trait]
impl Khudib for KhudibService {

    //type CreateKhudibRequestStreamStream=mpsc::Receiver<Result<KResponse,Status>>;
    
    //// implementation for rpc call
        //async fn create_khudib_request_stream(&self, request: Request<KRequest>) -> Result<Response<Self::CreateKhudibRequestStreamStream>, Status> {
            
            //// creating a queue or channel
            //let (mut tx, rx) = mpsc::channel(4);
            
            //// creating a new task
            //tokio::spawn(async move {
                
                //// looping and sending our response using stream
                //for _ in 0..4{
                    
                    //let payload = request.into_inner();
                    
                    //let kbrequest = Mkhudib {
                        //client_id: payload.client_id,
                        //server_id: payload.server_id,
                        //request_id: payload.request_id
                    //};
                    
                    //self.kbrequests.lock().unwrap().push(kbrequest.clone());
                    
                    //let message = KResponse {
                        //khudib: Some(kbrequest)
                    //};
                    
                    //// sending response to our channel
                    //tx.send(Ok(Response::new(message))).await;
                //}
            //});
            
            //// returning our reciever so that tonic can listen on reciever and send the response to client
            //Ok(Response::new(rx))
        //}
    
    async fn create_khudib_request(&self, request: Request<KRequest>) -> Result<Response<KResponse>, Status> {

        println!("Received the request at {}:", Utc::now().format("%S%.6f"));

        let payload = request.into_inner();

        let kbrequest = Mkhudib {
            client_id: payload.client_id,
            server_id: payload.server_id,
            request_id: payload.request_id
        };


        self.kbrequests.lock().unwrap().push(kbrequest.clone());

        let message = KResponse {
            khudib: Some(kbrequest)
        };

        thread::sleep(time::Duration::from_secs(1));

        println!("Responded to the request at {}:", Utc::now().format("%S%.6f"));
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

    let khudib_service = KhudibService::default();

    Server::builder()
        .add_service(KhudibServer::new(khudib_service))
        .serve(add)
        .await?;

    Ok(())
}
