use async_std::io;
use async_std::task;
use async_std::net::UdpSocket;

fn main() -> io::Result<()> {
    task::block_on(async_run())
}


async fn async_run() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:6000").await?;
    println!("Listening on {}", socket.local_addr()?);
    let mut buf = vec![0; 1024];
    let mut cnt = 0;
    loop {
        let (n, peer) = socket.recv_from(&mut buf).await?;
        cnt = cnt + 1;
        println!("{}: Recv {} bytes from {}", cnt, n, peer);
        let message : &[u8] = b"Received!!";
        socket.send_to(message, peer).await?;
        if cnt > 5 { break; }
    } 
    Ok(())
}


