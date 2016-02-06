//! Gmail inbox.

use google_gmail1::{Gmail, Error, Message, ModifyMessageRequest};
use hyper::Client as HttpClient;
use inth_oauth2::provider::Google;

use authenticator::Authenticator;
use token_cache::TokenCache;

pub use google_gmail1::Scope;

/// Gmail inbox client.
#[allow(missing_debug_implementations)]
pub struct Inbox<A: Authenticator<Google>> {
    gmail: Gmail<HttpClient, TokenCache<A>>,
}

impl<A: Authenticator<Google>> Inbox<A> {
    /// Creates a Gmail inbox client.
    ///
    /// `TokenCache::authenticate` should already have been called.
    pub fn new(http: HttpClient, token_cache: TokenCache<A>) -> Self {
        Inbox {
            gmail: Gmail::new(http, token_cache),
        }
    }

    /// Finds the first message in the inbox matching a query.
    pub fn find(&self, q: &str) -> Result<Option<Message>, Error> {
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
