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
            _ => (Scheme::default_scheme(), 443 as u16),
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
}
