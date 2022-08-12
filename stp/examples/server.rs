use stp::server::{StpConnection, StpServer};
use thiserror::Error;

#[derive(Debug, Error)]
enum ProcessError {
    #[error("Error while receiving request: {0}.")]
    WhenRecv(#[from] stp::error::RecvError),
    #[error("Error while sending response: {0}")]
    WhenSend(#[from] stp::error::SendError),
}

type ProcessResult = Result<(), ProcessError>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = StpServer::bind("127.0.0.1:5555").await?;
    loop {
        let conn = server.accept().await?;
        tokio::spawn(process_connection(conn));
    }
}

async fn process_connection(conn: StpConnection) -> ProcessResult {
    let req = conn.recv_request().await?;
    assert_eq!(req, "Hello, server");
    conn.send_response("Hello, client").await?;
    Ok(())
}
