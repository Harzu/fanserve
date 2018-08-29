#[derive(Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: String,
    pub protocol: String,
    pub path_static: String
}

impl ServerConfig {
    pub fn new(host: &str, port: &str, protocol: &str, directory: &str) -> Self {
        ServerConfig {
            host: host.to_string(),
            port: port.to_string(),
            protocol: protocol.to_string(),
            path_static: directory.to_string()
        }
    }
}