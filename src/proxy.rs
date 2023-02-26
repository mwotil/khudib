use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::SocketAddr;
use std::os::unix::io::{AsRawFd, RawFd};
use libc::{c_int, setsockopt, SOL_IP, IP_TRANSPARENT};

async fn handle_client(mut client_stream: TcpStream) {
    let mut buffer = [0; 1024];
    let read_bytes = client_stream.read(&mut buffer).await.unwrap();
    let request = String::from_utf8_lossy(&buffer[..read_bytes]);
    println!("Received request from client: {}", request);

    // Intercept and modify request here
    let modified_request = request.replace("original", "modified");
    println!("Modified request: {}", modified_request);

    // Connect to target server and send modified request
    let server_addr = "127.0.0.1:9000".parse::<SocketAddr>().unwrap();
    let mut server_stream = TcpStream::connect(server_addr).await.unwrap();
    server_stream.write_all(modified_request.as_bytes()).await.unwrap();

    // Forward response from target server to client
    let mut server_buffer = [0; 1024];
    let server_read_bytes = server_stream.read(&mut server_buffer).await.unwrap();
    let response = &server_buffer[..server_read_bytes];
    client_stream.write_all(response).await.unwrap();
}

fn set_transparent(fd: RawFd) -> Result<(), std::io::Error> {
    let enable: c_int = 1;
    let result = unsafe {
        setsockopt(fd, SOL_IP, IP_TRANSPARENT, &enable as *const c_int as _, std::mem::size_of::<c_int>() as u32)
    };
    if result == -1 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:9000").await.unwrap();
    println!("Target server listening on 127.0.0.1:9000");

    // Make the proxy transparent
    let proxy_addr = "127.0.0.1:9000".parse::<SocketAddr>().unwrap();
    let proxy_socket = TcpStream::connect(proxy_addr).await.unwrap().as_raw_fd();
    set_transparent(proxy_socket).expect("Failed to set IP_TRANSPARENT option");

    loop {
        let (client_stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_client(client_stream).await;
        });
    }
}

