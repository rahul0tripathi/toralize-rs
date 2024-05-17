use std::mem;
use std::os::raw::c_char;

const AF_INET: u8 = 2;
const SOCK_STREAM: u8 = 1;

#[repr(C)]
pub struct SUnB {
    s_b1: std::os::raw::c_uchar,
    s_b2: std::os::raw::c_uchar,
    s_b3: std::os::raw::c_uchar,
    s_b4: std::os::raw::c_uchar,
}

#[repr(C)]
pub struct SUnW {
    s_w1: std::os::raw::c_ushort,
    s_w2: std::os::raw::c_ushort,
}

#[repr(C)]
pub union SUn {
    s_un_b: std::mem::ManuallyDrop<SUnB>,
    s_un_w: std::mem::ManuallyDrop<SUnW>,
    s_addr: std::os::raw::c_ulong,
}

#[repr(C)]
pub struct InAddr {
    s_un: SUn,
}

#[repr(C)]
pub struct Socket {
    sin_family: std::os::raw::c_short,
    sin_port: std::os::raw::c_ushort,
    sin_addr: InAddr,
}
// 142.250.70.46

extern "C" {
    fn ntohs(netshort: std::os::raw::c_uint) -> std::os::raw::c_uint;
    fn inet_addr(cp: *const c_char) -> std::os::raw::c_uint;
    fn connect(socket: std::os::raw::c_int, sockaddr: *mut Socket, len: std::os::raw::c_uint) -> std::os::raw::c_int;
    fn socket(domain: std::os::raw::c_int, sock_type: std::os::raw::c_int,  protocol:std::os::raw::c_int) -> std::os::raw::c_int;
    fn perror(msg: *const c_char);

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
                    s_addr: addr as std::os::raw::c_ulong
                }
            },
        };

        let sock = socket(AF_INET as std::os::raw::c_int, SOCK_STREAM as std::os::raw::c_int, 0);
        println!("sock {}", sock);

        let size  =  mem::size_of::<Socket>();
        println!("struct size {}", size);

        if connect(sock, ip, size as std::os::raw::c_uint)<0 {
            perror(c"Failed to connect ".as_ptr());
            return;
        }

        println!("successfully connected")
    }
}
