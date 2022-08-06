use crate::error::{ConnectError, ConnectResult, RecvError, SendError};
use tokio::net::{TcpStream, ToSocketAddrs};
use thiserror::Error;

pub struct StpClient {
    stream: TcpStream,
}

impl StpClient {
    pub async fn connect<Addrs>(addr: Addrs) -> ConnectResult<Self>
    where
        Addrs: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addr).await?;
        Self::try_handshake(stream).await
    }

    async fn try_handshake(s: TcpStream) -> ConnectResult<Self> {
        super::write_all_async(&s, b"clnt").await?;
        let mut buf = [0; 4];
        super::read_exact_async(&s, &mut buf).await?;
        if &buf != b"serv" {
            let msg = format!("received: {:?}", buf);
            return Err(ConnectError::BadHandshake(msg));
        }
        Ok(Self { stream: s })
    }

    /// Send request to connected STP server.
    pub async fn send_request<R: AsRef<str>>(&self, req: R) -> RequestResult {
        super::send_string(req, &self.stream).await?;
        let response = super::recv_string(&self.stream).await?;
        Ok(response)
    }
}

pub type RequestResult = Result<String, RequestError>;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error(transparent)]
    Send(#[from] SendError),
    #[error(transparent)]
    Recv(#[from] RecvError),
}
