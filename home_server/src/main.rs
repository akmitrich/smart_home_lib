use std::{
    error::Error,
    sync::{Arc, RwLock},
    thread,
};

mod request_handler;
use request_handler::{Handler, Request};
use smart_home::home::Home;
use stp::{
    error::ConnectError,
    server::{StpConnection, StpServer},
};

fn main() -> Result<(), Box<dyn Error>> {
    let home = Arc::new(RwLock::new(Home::restore()));
    let addr = String::from("127.0.0.1:4083");
    let server = StpServer::bind(addr)?;
    for connection in server.incoming() {
        work_with(connection, home.clone())?;
    }
    Ok(())
}

fn work_with(
    connection_result: Result<StpConnection, ConnectError>,
    home_ptr: Arc<RwLock<Home>>,
) -> Result<(), Box<dyn Error>> {
    let connection = connection_result?;
    let addr = match connection.peer_addr() {
        Ok(a) => a.to_string(),
        Err(_) => String::from("Unknown addr"),
    };
    println!("connection from: {}", addr);

    thread::spawn(move || {
        if handle_connection(connection, home_ptr).is_err() {
            eprintln!("Client disconnected: {}", addr);
        }
    });
    Ok(())
}

fn handle_connection(
    mut conn: StpConnection,
    home: Arc<RwLock<Home>>,
) -> Result<(), Box<dyn Error>> {
    let mut handler = Handler::new(home);
    loop {
        let req_str = conn.recv_request()?;
        let mut req = Request::new(&req_str);
        conn.send_response(handler.respond(&mut req))?;
    }
}
