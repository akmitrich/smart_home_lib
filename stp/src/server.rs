use crate::error::{ConnectError, ConnectResult, RecvResult, SendResult};
use std::io;
use std::net::SocketAddr;
use thiserror::Error;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

pub struct StpServer {
    tcp: TcpListener,
}

pub type BindResult = Result<StpServer, BindError>;

#[derive(Debug, Error)]
pub enum BindError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

impl StpServer {
    pub async fn bind<Addrs>(addrs: Addrs) -> BindResult
    where
        Addrs: ToSocketAddrs,
    {
        let tcp = TcpListener::bind(addrs).await?;
        Ok(Self { tcp })
    }

    /// Blocking iterator for incoming connections.
    pub async fn accept(&self) -> ConnectResult<StpConnection> {
        let (stream, _) = self.tcp.accept().await?;
        Self::try_handshake(stream).await
    }

    async fn try_handshake(stream: TcpStream) -> ConnectResult<StpConnection> {
        let mut buf = [0; 4];
        super::read_exact_async(&stream, &mut buf).await?;
        if &buf != b"clnt" {
            let msg = format!("received: {:?}", buf);
            return Err(ConnectError::BadHandshake(msg));
        }
        super::write_all_async(&stream, b"serv").await?;
        Ok(StpConnection { stream })
    }
}

pub struct StpConnection {
    stream: TcpStream,
}

impl StpConnection {
    pub async fn send_response<Resp: AsRef<str>>(&self, response: Resp) -> SendResult {
        super::send_string(response, &self.stream).await
    }

    pub async fn recv_request(&self) -> RecvResult {
        crate::recv_string(&self.stream).await
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.stream.peer_addr()
    }
}
