use std::net::IpAddr;

pub enum Auth {
    NoAuth,
    Ldap,
    UserPass { username: String, password: String },
}

pub struct Config {
    pub addr: String,
    /// the revproxy's IP address -- to be sent
    /// in reply packets. (required for UDP)
    pub pub_addr: Option<IpAddr>,
    pub auth: Auth,
    pub timeout: usize,
    pub resolve_dns: bool,
    pub enable_udp: bool,
}

impl Config {
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

    /// reads config from env
    pub fn new() -> Self {
        let host = Self::get_env("REVPROXY_SOCKS5_HOST", "127.0.0.1".into());
        let port: u16 = Self::get_env("REVPROXY_SOCKS5_PORT", "1080".into())
            .parse()
            .expect("PORT is not a number");
        let pub_addr =
            Self::get_env_opt("REVPROXY_PUB_ADDR").map(|x| x.parse().expect("Invalid IP address"));
        let auth_method = Self::get_env("REVPROXY_AUTH_METHOD", "userpass".into());
        let auth = match auth_method.as_str() {
            "ldap" => Auth::Ldap,
            "no_auth" => Auth::NoAuth,
            "userpass" => {
                let username = Self::get_env("REVPROXY_AUTH_USER", "user".into());
                let password = Self::get_env("REVPROXY_AUTH_PASS", "pass".into());
                Auth::UserPass { username, password }
            }
            _ => panic!(
                "Invalid auth method \"{auth_method}\": Allowed are `ldap`, `no_auth`, `userpass`."
            ),
        };
        let timeout: usize = Self::get_env("REVPROXY_TIMEOUT", "10".into())
            .parse()
            .expect("TIMEOUT must be an integer");
        let enable_udp = Self::get_env_bool("REVPROXY_SOCKS5_ENABLE_UDP").unwrap_or(false);
        let resolve_dns = Self::get_env_bool("REVPROXY_SOCKS5_RESOLVE_DNS").unwrap_or(true);

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
