use std::*;
mod types;

use types::*;

extern "C" {
    fn ntohs(netshort: std::os::raw::c_uint) -> os::raw::c_uint;
    fn inet_addr(cp: *const libc::c_char) -> std::os::raw::c_uint;
    fn socket(
        domain: std::os::raw::c_int,
        sock_type: std::os::raw::c_int,
        protocol: std::os::raw::c_int,
    ) -> std::os::raw::c_int;
    fn connect(
        socket: std::os::raw::c_int,
        sockaddr: *mut Socket,
        len: std::os::raw::c_uint,
    ) -> std::os::raw::c_int;
    fn perror(msg: *const libc::c_char);
}

fn main() {
    unsafe {
        let p: u16 = 8484;
        let port = ntohs(p as std::os::raw::c_uint);
        println!("ntohs port: {}", port);

        let addr = inet_addr(c"127.0.0.1".as_ptr());
        println!("ipv4 addr: {}", addr);

        let ip: *mut Socket = &mut Socket {
            sin_family: AF_INET as std::os::raw::c_short,
            sin_port: port as std::os::raw::c_ushort,
            sin_addr: InAddr {
                s_un: SUn {
                    s_addr: addr as std::os::raw::c_ulong,
                },
            },
        };

        let sock = socket(
            AF_INET as std::os::raw::c_int,
            SOCK_STREAM as std::os::raw::c_int,
            0,
        );
        println!("sock {}", sock);

        let size = mem::size_of::<Socket>();
        println!("struct size {}", size);

        if connect(sock, ip, size as std::os::raw::c_uint) < 0 {
            perror(c"Failed to connect ".as_ptr());
            return;
        }

        println!("successfully connected")
    }
}
