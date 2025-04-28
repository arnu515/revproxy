use crate::config::Auth::*;
use fast_socks5::server::Authentication;

pub struct Auth(pub crate::config::Auth);

#[async_trait::async_trait]
impl Authentication for Auth {
    // TODO: fill this with something useful
    type Item = ();

    async fn authenticate(&self, credentials: Option<(String, String)>) -> Option<Self::Item> {
        match &self.0 {
            NoAuth => Some(()),
            UserPass { username, password } => credentials.and_then(|(user, pass)| {
                if &user == username && &pass == password {
                    Some(())
                } else {
                    None
                }
            }),
            Ldap => todo!(),
        }
    }
}
