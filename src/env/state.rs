use super::env::Env;

#[derive(Clone)]
pub struct AppState {
    pub host: String,
    pub port: u16,
    pub address: String,
}

impl AppState {
    pub fn from_env() -> Self {
        let env = Env::new();

        Self {
            host: env.host.into_owned(),
            port: env.port,
            address: env.address.into_owned(),
        }
    }
}
