use failure::Error;
use imap::client;
use imap_proto::{self, Response};
use native_tls::{TlsConnector, TlsStream};
use std::net::TcpStream;
use std::time::Duration;

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
    let client = client::Client::connect((imap.host, imap.port))?;
    let mut client = client.secure(imap.host, tls)?;
    client.login(imap.username, imap.password)?;
    Ok(client)
}

pub fn search(
    client: &mut Client, mailbox: &str, query: &str
) -> Result<Vec<u32>, Error> {
    client.select(mailbox)?;
    let res = client.run_command_and_read_response(&format!("SEARCH {}", query))?;
    match imap_proto::parse_response(&res).to_result()? {
        Response::IDs(ids) => Ok(ids),
        response => Err(format_err!("search response {:?}", response)),
    }
}

pub fn idle_search(
    client: &mut Client, mailbox: &str, query: &str, timeout: Duration
) -> Result<Vec<u32>, Error> {
    loop {
        let ids = search(client, mailbox, query)?;
        if ids.is_empty() {
            // FIXME: wait_timeout provides no indication of whether the
            // mailbox changed or the timeout was hit, so we have no way to
            // exit this loop.
            client.idle()?.wait_timeout(timeout)?;
        } else {
            return Ok(ids);
        }
    }
}
