use std::default::Default;
use crate::domain::model::url::Url;
use std::net::SocketAddr;
use std::str::FromStr;

#[derive(Deserialize, Debug, Clone)]
pub struct WebServer {
    #[serde(default = "WebServer::default_base_url")]
    base_url: Url,
    #[serde(default = "WebServer::default_bind_address")]
    bind_address: SocketAddr,
}

impl WebServer {
    pub fn get_base_url(&self) -> &Url {
        return &self.base_url;
    }

    pub fn get_bind_address(&self) -> &SocketAddr {
        return &self.bind_address;
    }

    fn default_base_url() -> Url {
        return Url::from_str("http://127.0.0.1:8888").unwrap();
    }

    fn default_bind_address() -> SocketAddr {
        return SocketAddr::from_str("127.0.0.1:8888").unwrap();
    }
}

impl Default for WebServer {
    fn default() -> Self {
        return WebServer {
            base_url: Self::default_base_url(),
            bind_address: Self::default_bind_address(),
        }
    }
}
