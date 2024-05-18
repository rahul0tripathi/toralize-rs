use std::io::{Read, Write};
use std::net::{Ipv4Addr, TcpStream};
use std::os::fd::AsRawFd;
use std::str::FromStr;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

const PROXY_ADDR: &str = "127.0.0.1";
const PROXY_PORT: u32 = 9050;


pub fn proxy() {
    let mut stream = TcpStream::connect(format!("{}:{}", PROXY_ADDR, PROXY_PORT)).unwrap();
    stream.set_nodelay(true).expect("TODO: panic message");
    let user_id: &[u8] = "W".as_bytes();
    let mut socks4Frame: Vec<u8> = Vec::new();
    socks4Frame.write_u8(4).expect("write VN");
    socks4Frame.write_u8(1).expect("write CD");
    let mut port: u16 = 80;

    let mut addr = Ipv4Addr::from_str("142.250.183.174").unwrap();


    socks4Frame.write_u16::<BigEndian>(port).expect("write_u16");
    socks4Frame.write_u32::<BigEndian>(addr.into()).expect("write_u32");
    socks4Frame.write_all(user_id).expect("write_all");
    socks4Frame.write_u8(0).expect("write u8");
    println!("{:?}", socks4Frame.as_slice());

    stream.write_all(&socks4Frame).expect("TODO: failed to write to sock");

    let mut response = [0u8; 8];
    stream.read_exact(&mut response).unwrap();
    let mut response = &response[..];

    if response.read_u8().unwrap() != 0 {
        panic!("invalid response version");
    }

    println!("{:?}", response);
}


#[cfg(test)]
mod tests {
    use crate::tor_proxy::proxy;

    #[test]
    fn test_conn() {
        proxy();
    }
}