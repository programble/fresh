//! Gmail inbox.

use std::thread;
use std::time::Duration;

use google_gmail1::{Gmail, Error, Message, MessagePart, ModifyMessageRequest};
use hyper::Client as HttpClient;
use inth_oauth2::provider::google::Installed;

use authenticator::Authenticator;
use token_cache::TokenCache;

pub use google_gmail1::Scope;

/// Gmail inbox client builder.
#[allow(missing_debug_implementations)]
pub struct InboxBuilder<A: Authenticator<Installed>> {
    token_cache: TokenCache<A>,
    http_client: Option<HttpClient>,
    find_tries: Option<u32>,
    find_delay: Option<Duration>,
}

impl<A: Authenticator<Installed>> InboxBuilder<A> {
    /// Creates a Gmail inbox client builder.
    ///
    /// `TokenCache::authenticate` should already have been called.
    pub fn new(token_cache: TokenCache<A>) -> Self {
        InboxBuilder {
            token_cache: token_cache,
            http_client: None,
            find_tries: None,
            find_delay: None,
        }
    }

    /// Sets the HTTP client.
    pub fn http_client(mut self, http_client: HttpClient) -> Self {
        self.http_client = Some(http_client);
        self
    }

    /// Sets the number of find tries.
    pub fn find_tries(mut self, tries: u32) -> Self {
        self.find_tries = Some(tries);
        self
    }

    /// Sets the delay between find tries.
    pub fn find_delay(mut self, delay: Duration) -> Self {
        self.find_delay = Some(delay);
        self
    }

    /// Creates a Gmail inbox client.
    pub fn finalize(self) -> Inbox<A> {
        let http_client = self.http_client.unwrap_or_else(Default::default);
        Inbox {
            gmail: Gmail::new(http_client, self.token_cache),
            find_tries: self.find_tries.unwrap_or(1),
            find_delay: self.find_delay.unwrap_or(Duration::new(0, 0)),
        }
    }
}

/// Gmail inbox client.
#[allow(missing_debug_implementations)]
pub struct Inbox<A: Authenticator<Installed>> {
    gmail: Gmail<HttpClient, TokenCache<A>>,
    find_tries: u32,
    find_delay: Duration,
}

impl<A: Authenticator<Installed>> Inbox<A> {
    /// Creates a Gmail inbox client.
    ///
    /// Convenience method for `InboxBuilder::new(token_cache).finalize()`.
    pub fn new(token_cache: TokenCache<A>) -> Self {
        InboxBuilder::new(token_cache).finalize()
    }

    fn _find(&self, q: &str) -> Result<Option<Message>, Error> {
        let (_, list) = try! {
            self.gmail.users()
                .messages_list("me")
                .add_label_ids("INBOX")
                .max_results(1)
                .q(q)
                .doit()
        };

        let partial = match list.messages.and_then(|v| v.into_iter().next()) {
            Some(m) => m,
            None => return Ok(None),
        };

        let (_, full) = try! {
            self.gmail.users()
                .messages_get("me", partial.id.as_ref().unwrap())
                .doit()
        };

        Ok(Some(full))
    }

    /// Finds the first message in the inbox matching a query, retrying with delay.
    pub fn find(&self, q: &str) -> Result<Option<Message>, Error> {
        for i in (0..self.find_tries).rev() {
            let message = try!(self._find(q));
            if message.is_some() {
                return Ok(message);
            }
            if i > 0 { thread::sleep(self.find_delay); }
        }
        Ok(None)
    }

    /// Marks as read and archives a message.
    ///
    /// Requires `Scope::Modify`.
    pub fn archive(&self, message: &Message) -> Result<(), Error> {
        self.gmail.users()
            .messages_modify(
                ModifyMessageRequest {
                    add_label_ids: None,
                    remove_label_ids: Some(vec![
                        String::from("INBOX"),
                        String::from("UNREAD"),
                    ]),
                },
                "me",
                message.id.as_ref().unwrap()
            )
            .add_scope(Scope::Modify)
            .doit()
            .and(Ok(()))
    }
}

/// Message extension methods.
pub trait MessageExt {
    /// Find a message part by type.
    fn find_part_by_type(&self, mime_type: &str) -> Option<&MessagePart>;
}

impl MessageExt for Message {
    fn find_part_by_type(&self, mime_type: &str) -> Option<&MessagePart> {
        let is_payload = self.payload.as_ref()
            .and_then(|payload| payload.mime_type.as_ref())
            .map_or(false, |t| t == mime_type);

        if is_payload {
            self.payload.as_ref()
        } else {
            self.payload.as_ref()
                .and_then(|payload| payload.parts.as_ref())
                .map(|parts| parts.iter())
                .and_then(|mut iter| {
                    iter.find(|part| {
                        part.mime_type.as_ref().map_or(false, |t| t == mime_type)
                    })
                })
        }
    }
}
