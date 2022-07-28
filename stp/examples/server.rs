use stp::server::{StpConnection, StpServer};

#[derive(Debug)]
enum ProcessError {
    WhenRecv(stp::error::RecvError),
    WhenSend(stp::error::SendError),
}

impl std::fmt::Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error while interact with client.")
    }
}

impl std::error::Error for ProcessError {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = StpServer::bind("127.0.0.1:5555").await?;
    loop {
        let conn = server.accept().await?;
        tokio::spawn(process_connection(conn));
    }
    Ok(())
}

async fn process_connection(conn: StpConnection) -> Result<(), ProcessError> {
    let req = conn.recv_request().await.map_err(|e| ProcessError::WhenRecv(e))?;
    assert_eq!(req, "Hello, server");
    conn.send_response("Hello, client").await.map_err(|e| ProcessError::WhenSend(e))?;
    Ok(())
}
