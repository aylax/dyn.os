// sockets/rs

/*
 * # about project
 * cargo run --example main # for examples/main.rs
 * cargo test # for test
 * 
 * # test udp 
 * cargo run lynn # udp socket listen on port 6000
 * # echo "send sth..." > /dev/udp/127.0.0.1/6000
 * exec 8<>/dev/udp/127.0.0.1/6000 # link dev/udp to file descriptor :8
 * echo "send sth...">&8 # write to fd :8
 * cat <&8 # out :8 content
 * 
 * # tree proj dir
 * tree -aF -I 'target|.git' > .proj
 * 
 */
use async_std::io;
use async_std::task;
use async_std::net::UdpSocket;

fn main() -> io::Result<()> {
    task::block_on(async_run())
}

async fn async_run() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:6000").await?;
    println!("Listening on {}", socket.local_addr()?);
    let (mut buf, mut cnt) = (vec![0; 1024], 0);
    loop {
        let (n, peer) = socket.recv_from(&mut buf).await?;
        println!("{}: Recv {} bytes from {}", cnt, n, peer);
        let message : &[u8] = b"Received!!";
        socket.send_to(message, peer).await?;
        if cnt > 5 { break; } else { cnt += 1; }
    } 
    Ok(())
}

