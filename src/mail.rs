use failure::Error;
use imap::client;
use native_tls::{TlsConnector, TlsStream};
use std::net::TcpStream;

fn default_port() -> u16 { 993 }

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Imap<'a> {
    pub host: &'a str,

    #[serde(default = "default_port")]
    pub port: u16,

    pub username: &'a str,

    pub password: &'a str,
}

pub type Client = client::Client<TlsStream<TcpStream>>;

pub fn connect(tls: &TlsConnector, imap: &Imap) -> Result<Client, Error> {
    let tls = tls.clone(); // FIXME: imap should borrow this.
    let client = client::Client::connect((imap.host, imap.port))?;
    let mut client = client.secure(imap.host, tls)?;
    client.login(imap.username, imap.password)?;
    Ok(client)
}
