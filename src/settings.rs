use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::Deserialize;
use std::net::{Ipv4Addr, SocketAddr};

#[derive(Deserialize)]
pub struct Settings {
    address: String,
    port: u16,
    pub github_private_key: String,
}

impl Settings {
    pub fn get() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::new("config.toml", FileFormat::Toml))
            .add_source(Environment::with_prefix("COG"))
            .build()?;

        config.try_deserialize()
    }

    pub fn address(&self) -> SocketAddr {
        let addr: Ipv4Addr = self.address.parse().expect("Valid socket address");
        SocketAddr::new(addr.into(), self.port)
    }
}

#[cfg(test)]
mod test {
    use crate::settings::Settings;

    #[test]
    fn should_get_settings() {
        let settings = Settings::get();
        assert!(settings.is_ok());
        assert_eq!(settings.unwrap().address().port(), 8080);
    }
}
