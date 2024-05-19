use std::net::{Ipv4Addr, TcpStream};
use std::os::fd::AsRawFd;
use std::str::FromStr;
use libc::c_char;

mod types;
mod tor_proxy;

pub use types::*;
use tor_proxy::*;


redhook::hook! {
    unsafe fn connect(socket: libc::c_int, sockaddr: *mut libc::sockaddr, len: libc::c_uint) -> libc::c_int => conn {
        let data = (*sockaddr).sa_data;
        // unhandled ipv6
        if data[13] != 0 {
            return -1;
        }
        let port_bytes: [u8;2] = [data[0] as u8, data[1] as u8];
        let port_num = u16::from_be_bytes(port_bytes);

        let addr_bytes: [u8; 4] = data[2..6].iter().map(|&b| b as u8).collect::<Vec<u8>>().try_into().expect("failed to parse ipv4 address");
        let addr_native = Ipv4Addr::from(addr_bytes);

        println!("routing via tor proxy fd: {:?} dst: {}", socket, format!("{}:{}", addr_native, port_num));

        let mut stream = TcpStream::connect(format!("{}:{}", PROXY_ADDR, PROXY_PORT)).expect("failed to connect to proxy");


        if dup2(stream.as_raw_fd(), socket) < 0 {
            println!("failed to call dup2 wit params old: {:?} new: {:?}",stream.as_raw_fd(), socket);
            perror(c"dup2 perror".as_ptr());
            return -1;
        }

        let res = proxy(port_num,addr_native,stream);
        match res {
        Ok(_) => {},
        Err(err) => {
                println!("failed to connect to proxy {}", err);
                return -1;
            }
        }

        println!("bridged socks");
        return 0;
    }
}


