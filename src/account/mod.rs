//! Accounts for which passwords can be reset.

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;

use hyper::{self, Client};
use hyper::status::StatusCode;

/// An account whose password can be reset.
pub trait Account {
    /// Returns a URL at which the user can log in to the account.
    fn login_url(&self) -> String;

    /// Initiates the password reset flow, usually through a "forgot password" form.
    fn initiate_reset(&self, client: &Client) -> Result<(), AccountError>;
}

/// An unexpected HTTP status code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusError(pub StatusCode);

impl Display for StatusError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "unexpected status: {}", self.0)
    }
}

impl Error for StatusError {
    fn description(&self) -> &str { "unexpected status" }
}

/// An unexpected markup error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarkupError {
    /// Missing element.
    MissingElement(String),

    /// Missing attribute.
    MissingAttr(String),
}

impl Display for MarkupError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            MarkupError::MissingElement(ref elem) => write!(f, "missing element: {}", elem),
            MarkupError::MissingAttr(ref attr) => write!(f, "missing attribute: {}", attr),
        }
    }
}

impl Error for MarkupError {
    fn description(&self) -> &str {
        match *self {
            MarkupError::MissingElement(_) => "missing element",
            MarkupError::MissingAttr(_) => "missing attribute",
        }
    }
}

macro_rules! error_enum {
    (
        #[$meta:meta]
        pub enum $ident:ident {
            $(
                #[$vmeta:meta]
                $variant:ident($ty:ty),
            )+
        }
    ) => {
        #[$meta]
        #[derive(Debug)]
        pub enum $ident {
            $(
                #[$vmeta]
                $variant($ty),
            )+
        }

        impl Display for $ident {
            fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
                match *self {
                    $($ident::$variant(ref err) => write!(f, "{}", err),)+
                }
            }
        }

        impl Error for $ident {
            fn description(&self) -> &str {
                match *self {
                    $($ident::$variant(ref err) => err.description(),)+
                }
            }

            fn cause(&self) -> Option<&Error> {
                match *self {
                    $($ident::$variant(ref err) => Some(err),)+
                }
            }
        }

        $(
            impl From<$ty> for $ident {
                fn from(err: $ty) -> Self {
                    $ident::$variant(err)
                }
            }
        )+
    }
}

error_enum! {
    /// An error that can occur during the password reset flow.
    pub enum AccountError {
        /// An IO error.
        Io(io::Error),

        /// A Hyper error.
        Hyper(hyper::Error),

        /// An unexpected status error.
        Status(StatusError),

        /// An unexpected markup error.
        Markup(MarkupError),
    }
}

pub use self::hacker_news::HackerNews;
mod hacker_news;
