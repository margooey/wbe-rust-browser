use socket2::{Domain, Protocol, Socket, Type};
use std::mem::MaybeUninit;
use std::net::{SocketAddr, ToSocketAddrs};

pub const HOMEPAGE: &'static str = "https://browser.engineering";

#[derive(Debug)]
enum Scheme {
    HTTP,
    HTTPS,
}

impl Scheme {
    pub fn default_scheme() -> Scheme {
        Scheme::HTTP
    }
}

#[derive(Debug)]
pub struct Url {
    scheme: Scheme,
    host: String,
    path: String,
    port: u16, // 0-65535
}

impl Url {
    pub fn new(url: &str) -> Self {
        let (scheme_str, remainder) = url.split_once("://").unwrap();
        let (scheme, mut port) = match scheme_str {
            "80" => (Scheme::HTTP, 80 as u16),
            "443" => (Scheme::HTTPS, 443 as u16),
            _ => (Scheme::default_scheme(), 80 as u16),
        };

        let (host, remainder) = remainder.split_once("/").unwrap_or((remainder, ""));
        let path = "/".to_owned() + remainder;
        if host.contains(":") {
            let (_, port_str) = host.split_once(":").unwrap();
            port = port_str.to_string().parse::<u16>().unwrap();
        }

        Self {
            scheme,
            host: host.to_owned(),
            path,
            port: port,
        }
    }
    pub fn request(self) {
        let s = Socket::new(
            Domain::IPV4,        // AF_INET
            Type::STREAM,        // SOCK_STREAM
            Some(Protocol::TCP), // IPPROTO_TCP
        )
        .unwrap();
        let addresses: Vec<SocketAddr> = format!("{}:{}", &self.host, &self.port)
            .to_socket_addrs()
            .unwrap()
            .collect(); // Returns a vector containing the IPV6 (index 0) and IPV4 (index 1) addresses for the url
        s.connect(&addresses[1].into()).unwrap();
        let mut request = format!("GET {} HTTP/1.0\r\n", self.path);
        request += format!("Host: {}\r\n", self.host).as_str();
        request += "\r\n";
        s.send(request.as_bytes()).unwrap(); // UTF-8 encoding
        let mut buf = [MaybeUninit::<u8>::uninit(); 1024]; // Uninitialized buffer wrapped in MaybeUninit<u8>
        loop {
            let bytes = s.recv(&mut buf).unwrap();
            if bytes == 0 {
                break;
            }
            let initialized_bytes = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u8, bytes) };
            print!("{}", String::from_utf8_lossy(initialized_bytes));
        }
    }
}
