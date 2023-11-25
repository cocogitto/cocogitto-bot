use config::{Config, ConfigBuilder, ConfigError, File, FileFormat};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Deserialize)]
pub struct Settings {
    address: SocketAddr,
    port: u16,
}

impl Settings {
    pub fn get() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::new("config.toml", FileFormat::Toml))
            .build()?;

        config.try_deserialize()
    }

    pub fn address(&self) -> SocketAddr {
        let mut addr = self.address.clone();
        addr.set_port(self.port);
        addr
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
