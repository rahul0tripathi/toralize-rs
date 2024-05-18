use std::os;
use redhook::real;
mod types;
mod tor_proxy;

pub use types::*;



redhook::hook! {
    unsafe fn connect(socket: libc::c_int, sockaddr: *mut Socket, len: libc::c_uint) -> libc::c_int => conn {
        println!("called custom connect {} {} {}", socket, (*sockaddr).sin_port,len );

        return real!(connect)(socket,sockaddr,len);
    }
}


