use std::os;

pub const AF_INET: u8 = 2;

pub const SOCK_STREAM: u8 = 1;

#[repr(C)]
#[derive(Debug)]
pub struct SUnB {
    s_b1: libc::c_uchar,
    s_b2: libc::c_uchar,
    s_b3: libc::c_uchar,
    s_b4: libc::c_uchar,
}

#[repr(C)]
#[derive(Debug)]
pub struct SUnW {
    s_w1: libc::c_ushort,
    s_w2: libc::c_ushort,
}

#[repr(C)]
pub struct SUn {
    pub s_addr: libc::c_ulong,
}

#[repr(C)]
pub struct InAddr {
    pub s_un: SUn,
}

#[repr(C)]
pub struct Socket {
    pub sin_family: libc::c_short,
    pub sin_port: libc::c_ushort,
    pub sin_addr: InAddr,
}

extern "C" {
    pub fn ntohs(netshort: std::os::raw::c_uint) -> os::raw::c_uint;
    pub fn inet_addr(cp: *const libc::c_char) -> std::os::raw::c_uint;
    pub fn socket(
        domain: std::os::raw::c_int,
        sock_type: std::os::raw::c_int,
        protocol: std::os::raw::c_int,
    ) -> std::os::raw::c_int;
    pub fn connect(
        socket: std::os::raw::c_int,
        sockaddr: *mut Socket,
        len: std::os::raw::c_uint,
    ) -> std::os::raw::c_int;
    pub fn perror(msg: *const libc::c_char);
    pub fn dup2(old: os::raw::c_int , new: os::raw::c_int) -> os::raw::c_int;
    pub fn dup(old: os::raw::c_int ) -> os::raw::c_int;
    pub fn inet_ntoa(ip: libc::c_ulong) -> *const libc::c_char;
}



pub const PROXY_ADDR: &str = "127.0.0.1";
pub const PROXY_PORT: u32 = 9050;