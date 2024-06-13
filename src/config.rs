use std::env;
use std::net::IpAddr;
use std::sync::OnceLock;

#[derive(Debug, Clone)]
struct ServerConfig<T: Into<IpAddr>> {
    host: T,
    port: u16,
}

#[derive(Debug, Clone)]
struct DatabaseConfig {
    database_url: String,
    pool_size: usize,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub server: ServerConfig<[u8; 4]>,
    pub database: DatabaseConfig,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

impl Config {
    pub fn server_host(&self) -> &[u8; 4] {
        &self.server.host
    }

    pub fn server_port(&self) -> u16 {
        self.server.port
    }

    pub fn database_url(&self) -> &str {
        &self.database.database_url
    }

    pub fn database_pool_size(&self) -> usize {
        self.database.pool_size
    }
}

fn init_config() -> Config {
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    set_env_var("RUST_LOG", &log_level);

    let server_config = load_server_config();
    let database_config = load_data_config();

    Config {
        server: server_config,
        database: database_config,
    }
}

fn load_server_config() -> ServerConfig<[u8; 4]> {
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

    ServerConfig {
        host: server_host,
        port: server_port,
    }
}

fn load_data_config() -> DatabaseConfig {
    let database_url = env::var("DATABASE_URL").expect("undefined [DATABASE_URL]");
    let pool_size = env::var("POOL_SIZE")
        .expect("undefined [POOL_SIZE]")
        .parse::<usize>()
        .unwrap();

    DatabaseConfig {
        database_url,
        pool_size,
    }
}

pub async fn config() -> &'static Config {
    CONFIG.get_or_init(init_config)
}

fn set_env_var(key: &str, value: &str) {
    unsafe {
        env::set_var(key, value);
    }
}
