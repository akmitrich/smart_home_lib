use crate::error::{RecvError, RecvResult, SendResult};
use std::io;
use tokio::net::TcpStream;

pub mod client;
pub mod error;
pub mod server;

async fn read_exact_async(s: &TcpStream, buf: &mut [u8]) -> io::Result<()> {
    let mut have_read = 0;
    while have_read < buf.len() {
        s.readable().await?;
        match s.try_read(&mut buf[have_read..]) {
            Ok(0) => break,
            Ok(n) => {
                have_read += n;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

async fn write_all_async(s: &TcpStream, buf: &[u8]) -> io::Result<()> {
    let mut written = 0;

    while written < buf.len() {
        s.writable().await?;

        match s.try_write(&buf[written..]) {
            Ok(0) => break,
            Ok(n) => {
                written += n;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

async fn send_string<D: AsRef<str>>(d: D, w: &TcpStream) -> SendResult {
    let bytes = d.as_ref().as_bytes();
    let len = bytes.len() as u32;
    let len_bytes = len.to_be_bytes();
    write_all_async(w, &len_bytes).await?;
    write_all_async(w, bytes).await?;
    Ok(())
}

async fn recv_string(r: &TcpStream) -> RecvResult {
    let mut buf = [0; 4];
    read_exact_async(r, &mut buf).await?;
    let len = u32::from_be_bytes(buf);

    let mut buf = vec![0; len as _];
    read_exact_async(r, &mut buf).await?;
    String::from_utf8(buf).map_err(|_| RecvError::BadEncoding)
}
