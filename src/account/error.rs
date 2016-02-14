//! Account errors.

use std::error::Error;
use std::fmt::{Display, Formatter, Error as FmtError};
use std::io;
use std::string::FromUtf8Error;

use google_gmail1;
use hyper;
use hyper::status::StatusCode;
use rustc_serialize::base64::FromBase64Error;

/// An unexpected HTTP status code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusError {
    /// URL.
    pub url: String,

    /// Status code.
    pub status: StatusCode,
}

impl StatusError {
    /// Creates a `StatusError`.
    pub fn new(url: &str, status: StatusCode) -> Self {
        StatusError {
            url: String::from(url),
            status: status,
        }
    }
}

impl Display for StatusError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "unexpected status {} at {}", self.url, self.status)
    }
}

impl Error for StatusError {
    fn description(&self) -> &str { "unexpected status" }
}

/// An unexpected markup error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarkupError {
    /// Missing element.
    MissingElement {
        /// URL.
        url: String,

        /// Selector.
        selector: String,
    },

    /// Missing attribute.
    MissingAttr {
        /// URL.
        url: String,

        /// Selector.
        selector: String,

        /// Attribute.
        attr: String,
    },
}

impl MarkupError {
    /// Creates a `MarkupError::MissingElement`.
    pub fn missing_element(url: &str, selector: &str) -> Self {
        MarkupError::MissingElement {
            url: String::from(url),
            selector: String::from(selector),
        }
    }

    /// Creates a `MarkupError::MissingAttr`.
    pub fn missing_attr(url: &str, selector: &str, attr: &str) -> Self {
        MarkupError::MissingAttr {
            url: String::from(url),
            selector: String::from(selector),
            attr: String::from(attr),
        }
    }
}

impl Display for MarkupError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match *self {
            MarkupError::MissingElement { ref url, ref selector } => {
                write!(f, "missing element {} at {}", selector, url)
            },
            MarkupError::MissingAttr { ref url, ref selector, ref attr } => {
                write!(f, "missing attribute {} on {} at {}", attr, selector, url)
            }
        }
    }
}

impl Error for MarkupError {
    fn description(&self) -> &str {
        match *self {
            MarkupError::MissingElement { .. } => "missing element",
            MarkupError::MissingAttr { .. } => "missing attribute",
        }
    }
}

/// A Gmail message error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageError {
    /// No message matched the query.
    Missing(String),

    /// No message part of type.
    MissingPart(String),
}

impl Display for MessageError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match *self {
            MessageError::Missing(ref q) => {
                write!(f, "missing message matching query '{}'", q)
            },
            MessageError::MissingPart(ref t) => {
                write!(f, "missing message part of type '{}'", t)
            },
        }
    }
}

impl Error for MessageError {
    fn description(&self) -> &str {
        match *self {
            MessageError::Missing(_) => "missing message",
            MessageError::MissingPart(_) => "missing message part",
        }
    }
}

macro_rules! error_enum {
    (
        $(#[$meta:meta])+
        pub enum $ident:ident {
            $(
                #[$vmeta:meta]
                $variant:ident($ty:ty),
            )+
        }
    ) => {
        $(#[$meta])+
        #[derive(Debug)]
        pub enum $ident {
            $(
                #[$vmeta]
                $variant($ty),
            )+
        }

        impl Display for $ident {
            fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
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
    #[allow(variant_size_differences)]
    pub enum AccountError {
        /// A UTF-8 error.
        Utf8(FromUtf8Error),

        /// An IO error.
        Io(io::Error),

        /// A base64 error.
        Base64(FromBase64Error),

        /// A Hyper error.
        Hyper(hyper::Error),

        /// A Gmail error.
        Gmail(google_gmail1::Error),

        /// An unexpected status error.
        Status(StatusError),

        /// An unexpected markup error.
        Markup(MarkupError),

        /// A Gmail message error.
        Message(MessageError),
    }
}
