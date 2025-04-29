use std::net::IpAddr;

pub enum Auth {
    NoAuth,
    Ldap,
    UserPass { username: String, password: String },
}

pub struct Config {
    pub socks: SocksConfig,
    pub http: HttpConfig,
}

impl Config {
    pub fn new() -> Self {
        Self {
            socks: SocksConfig::new(),
            http: HttpConfig::new(),
        }
    }
}

pub struct SocksConfig {
    pub addr: String,
    /// the revproxy's IP address -- to be sent
    /// in reply packets. (required for UDP)
    pub pub_addr: Option<IpAddr>,
    pub auth: Auth,
    pub timeout: usize,
    pub resolve_dns: bool,
    pub enable_udp: bool,
}

impl SocksConfig {
    /// reads config from env
    pub fn new() -> Self {
        let host = get_env("REVPROXY_SOCKS_HOST", "127.0.0.1".into());
        let port: u16 = get_env("REVPROXY_SOCKS_PORT", "1080".into())
            .parse()
            .expect("PORT is not a number");
        let pub_addr =
            get_env_opt("REVPROXY_PUB_ADDR").map(|x| x.parse().expect("Invalid IP address"));
        let auth_method = get_env("REVPROXY_SOCKS_AUTH_METHOD", "userpass".into());
        let auth = match auth_method.as_str() {
            "ldap" => Auth::Ldap,
            "no_auth" => Auth::NoAuth,
            "userpass" => {
                let username = get_env("REVPROXY_SOCKS_AUTH_USER", "user".into());
                let password = get_env("REVPROXY_SOCKS_AUTH_PASS", "pass".into());
                Auth::UserPass { username, password }
            }
            _ => panic!(
                "Invalid auth method \"{auth_method}\": Allowed are `ldap`, `no_auth`, `userpass`."
            ),
        };
        let timeout: usize = get_env("REVPROXY_SOCKS_TIMEOUT", "10".into())
            .parse()
            .expect("TIMEOUT must be an integer");
        let enable_udp = get_env_bool("REVPROXY_SOCKS_ENABLE_UDP").unwrap_or(false);
        let resolve_dns = get_env_bool("REVPROXY_SOCKS_RESOLVE_DNS").unwrap_or(true);

        Self {
            addr: format!("{host}:{port}"),
            pub_addr,
            auth,
            timeout,
            resolve_dns,
            enable_udp,
        }
    }
}

pub struct HttpConfig {
    pub addr: String,
    pub https: Option<HttpsConfig>,
    pub auth: Auth,
}

impl HttpConfig {
    /// reads config from env
    pub fn new() -> Self {
        let host = get_env("REVPROXY_HTTP_HOST", "127.0.0.1".into());
        let port: u16 = get_env("REVPROXY_HTTP_PORT", "8080".into())
            .parse()
            .expect("PORT is not a number");
        let auth_method = get_env("REVPROXY_HTTP_AUTH_METHOD", "userpass".into());
        let auth = match auth_method.as_str() {
            "ldap" => Auth::Ldap,
            "no_auth" => Auth::NoAuth,
            "userpass" => {
                let username = get_env("REVPROXY_HTTP_AUTH_USER", "user".into());
                let password = get_env("REVPROXY_HTTP_AUTH_PASS", "pass".into());
                Auth::UserPass { username, password }
            }
            _ => panic!(
                "Invalid auth method \"{auth_method}\": Allowed are `ldap`, `no_auth`, `userpass`."
            ),
        };

        let https = match get_env_bool("REVPROXY_HTTPS") {
            Some(true) => Some(HttpsConfig::new()),
            _ => None,
        };

        Self {
            addr: format!("{host}:{port}"),
            https,
            auth,
        }
    }
}

pub struct HttpsConfig {
    pub host: String,
    pub port: u16,
    pub cert_path: String,
    pub key_path: String,
}

impl HttpsConfig {
    fn new() -> Self {
        let host = get_env("REVPROXY_HTTPS_HOST", "127.0.0.1".into());
        let port: u16 = get_env("REVPROXY_HTTPS_PORT", "4430".into())
            .parse()
            .expect("PORT is not a number");
        let cert_path =
            get_env_opt("REVPROXY_HTTPS_CERT").expect("CERT must be set if HTTPS is enabled");
        let key_path = get_env_opt("REVPROXY_HTTPS_CERT_KEY")
            .expect("CERT_KEY must be set if HTTPS is enabled");

        Self {
            host,
            port,
            cert_path,
            key_path,
        }
    }
}

#[inline]
fn get_env(key: &str, def: String) -> String {
    match std::env::var(key) {
        Ok(var) => var,
        Err(std::env::VarError::NotPresent) => def,
        Err(_) => panic!("Env var {key} is not unicode"),
    }
}

#[inline]
fn get_env_opt(key: &str) -> Option<String> {
    match std::env::var(key) {
        Ok(var) => Some(var),
        Err(std::env::VarError::NotPresent) => None,
        Err(_) => panic!("Env var {key} is not unicode"),
    }
}

#[inline]
fn get_env_bool(key: &str) -> Option<bool> {
    match std::env::var(key) {
        Ok(var) => Some(&var != "0"),
        Err(std::env::VarError::NotPresent) => None,
        Err(_) => panic!("Env var {key} is not unicode"),
    }
}
