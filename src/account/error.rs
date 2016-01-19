//! Account errors.

use std::error::Error;
use std::fmt::{Display, Formatter, Error as FmtError};
use std::io;

use hyper;
use hyper::status::StatusCode;

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
