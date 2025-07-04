use std::net::{AddrParseError, Incoming, IpAddr, Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr, SocketAddrV4, SocketAddrV6, TcpListener, TcpStream};

fn main() {
    println!("Hello, world!");
}

fn fn_struct() {
    //AddrParseError
    //Incoming
    //Ipv4Addr
    //Ipv6Addr
}

#[test]
fn fn_ipv4() {
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    println!("IP Address: {}", ip);
}

#[test]
fn fn_ipv6() {
    let ip = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    println!("IP Address: {}", ip);
}

#[test]
fn socket_addr_v4() {
    let socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8800);
    println!("Socket Address: {}", socket);
}

#[test]
fn socket_addr_v6() {
    let socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8800, 0, 0);
    println!("Socket Address: {}", socket);
}

fn handle_client(stream: TcpStream) {
    stream.set_nonblocking(true).unwrap();
    stream.shutdown(Shutdown::Both).unwrap()
}

#[test]
fn fn_tcp_listen() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8800")?;
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

#[test]
fn fn_tcp_listen_address() -> std::io::Result<()> {
    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 8800)),
        SocketAddr::from(([127, 0, 0, 1], 8801)),
    ];
    let listener = TcpListener::bind(&addrs[..])?;
    println!("Listening on: {}", listener.local_addr()?);
    Ok(())
}

//nc -w 5 127.0.0.1 8800
#[test]
fn fn_tcp_accept() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8088")?;
    match listener.accept() {
        Ok((_socket, addr)) => {
            println!("Accepted connection from: {}", addr);
        }
        Err(e) => {
            eprintln!("Failed to accept connection: {}", e);
            return Err(e);
        }
    }
    Ok(())
}
