use std::io;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, TcpStream};
use std::os::fd::AsRawFd;
use std::str::FromStr;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

const UNAME: &[u8] = "TORALIZERS".as_bytes();


pub fn proxy(dst_port: u16, dst_address: Ipv4Addr, mut stream: TcpStream) -> io::Result<()>  {
    stream.set_nodelay(true)?;

    /*
        socks4_frame: a socks4 frame where VN = 4, CD = 1, connect command and other fields,
        NULL is u8 with all 0s
        SOCKS request layout:
                +----+----+----+----+----+----+----+----+----+----+....+----+
                | VN | CD | DSTPORT |      DSTIP        | USERID       |NULL|
                +----+----+----+----+----+----+----+----+----+----+....+----+
                   1    1      2              4           variable       1
    */

    let mut socks4_frame: Vec<u8> = Vec::new();
    socks4_frame.write_u8(4)?;
    socks4_frame.write_u8(1)?;
    socks4_frame.write_u16::<BigEndian>(dst_port)?;
    socks4_frame.write_u32::<BigEndian>(dst_address.into())?;
    socks4_frame.write_all(UNAME)?;
    socks4_frame.write_u8(0)?;

    stream.write_all(&socks4_frame)?;

    let mut response = [0u8; 8];
    stream.read_exact(&mut response)?;
    let mut response = &response[..];

    // response VN should be 0
    if response.read_u8().unwrap() != 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid response version"));
    }

    /*
    read CD code from the response and parse as following:
            90: request granted
            91: request rejected or failed
            92: request rejected because the SOCKS server cannot connect to
                identd on the client
            93: request rejected because the client program and identd
                report different user-ids
    */
    if response[0] != 90 {
        return Err(io::Error::new(io::ErrorKind::Other, format!("connection failed with status code: {}", response[0])));
    }

    return Ok(());
}


#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, TcpStream};
    use std::str::FromStr;
    use crate::tor_proxy::{proxy};
    use crate::types::{ PROXY_ADDR, PROXY_PORT};

    #[test]
    fn test_conn() {
        let  port: u16 = 80;
        let  addr = Ipv4Addr::from_str("34.117.118.45").unwrap();
        let stream = TcpStream::connect(format!("{}:{}", PROXY_ADDR, PROXY_PORT)).unwrap();
        proxy(port,addr,stream).unwrap();
    }
}