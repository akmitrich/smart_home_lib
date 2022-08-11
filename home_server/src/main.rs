use std::{error::Error, sync::Arc};
use tokio::sync::RwLock;

mod request_handler;
use request_handler::{Handler, Request};
use smart_home::home::Home;
use stp::server::{StpConnection, StpServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let home = Arc::new(RwLock::new(Home::restore()));
    let addr = String::from("127.0.0.1:4083");
    let server = StpServer::bind(addr).await?;
    loop {
        let connection = server.accept().await?;
        work_with(connection, Arc::clone(&home))?;
    }
}

fn work_with(connection: StpConnection, home: Arc<RwLock<Home>>) -> Result<(), Box<dyn Error>> {
    let addr = match connection.peer_addr() {
        Ok(addr) => addr.to_string(),
        Err(_) => String::from("Unknown addr"),
    };
    println!("connection from: {}", addr);

    tokio::spawn(async move {
        if handle_connection(connection, home).await.is_err() {
            eprintln!("Client disconnected: {}", addr);
        }
    });
    Ok(())
}

async fn handle_connection(connection: StpConnection, home: Arc<RwLock<Home>>) -> Result<(), Box<dyn Error>> {
    let mut handler = Handler::new(home);
    loop {
        let req_str = connection.recv_request().await?;
        let mut req = Request::new(&req_str);
        connection.send_response(handler.respond(&mut req).await).await?;
    }
}
