use std::env;
use std::net::IpAddr;
use std::sync::OnceLock;

use dotenv::dotenv;

#[derive(Debug)]
struct ServerConfig<T: Into<IpAddr>> {
    host: T,
    port: u16,
}

#[derive(Debug)]
pub struct Config {
    pub server: ServerConfig<[u8; 4]>,
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();

impl Config {
    pub fn server_host(&self) -> &[u8; 4] {
        &self.server.host
    }

    pub fn server_port(&self) -> u16 {
        self.server.port
    }
}

fn init_config() -> Config {
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", &log_level);

    dotenv().ok();
    let server_url = env::var("SERVER_URL")
        .expect("undefined [SERVER_URL]")
        .split(".")
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    let server_host: [u8; 4] = [server_url[0], server_url[1], server_url[2], server_url[3]];

    let server_port = env::var("SERVER_PORT")
        .expect("undefined [SERVER_PORT]")
        .parse::<u16>()
        .unwrap();

    let server_config = ServerConfig {
        host: server_host,
        port: server_port,
    };

    Config {
        server: server_config,
    }
}
