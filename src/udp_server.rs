use std::net::UdpSocket;
use std::str;

pub fn serve(address: &str) -> anyhow::Result<()> {
    let server_socket = UdpSocket::bind(address)?;
    loop {
        let mut buffer = [0u8; 1024];
        let (size, src) = server_socket.recv_from(&mut buffer)?;
        log::debug!("Handling data from {}", src);
        print!("{}", str::from_utf8(&mut buffer[..size])?);
        server_socket.send_to(&buffer, src)?;
    }
}
