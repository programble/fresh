//! Account helpers.

use std::io::Read;

use google_gmail1::Message;
use hyper::client::{Client, Response};
use hyper::header::ContentType;
use hyper::status::StatusCode;
use inth_oauth2::provider::google::Installed;
use scraper::{Html, Selector, NodeRef};
use regex::{Regex, Captures};
use rustc_serialize::base64::FromBase64;
use url::form_urlencoded;

use authenticator::Authenticator;
use gmail::{Inbox, MessageExt};
use super::error::{AccountError, StatusError, MarkupError, MessageError};

/// Performs a `GET` request and returns `Err` if the response status is not `status`.
pub fn get_expect(
    client: &Client,
    url: &str,
    status: StatusCode,
) -> Result<Response, AccountError> {
    let request = client.get(url);
    let response = try!(request.send());
    if response.status != status {
        return Err(StatusError::new(url, response.status).into());
    }
    Ok(response)
}

/// Performs a `POST` request and returns `Err` if the response status is not `status`.
pub fn post_expect(
    client: &Client,
    url: &str,
    body_pairs: &[(&str, &str)],
    status: StatusCode,
) -> Result<Response, AccountError> {
    let body = form_urlencoded::serialize(body_pairs);
    let request = client.post(url)
        .header(ContentType::form_url_encoded())
        .body(&body);
    let response = try!(request.send());
    if response.status != status {
        return Err(StatusError::new(url, response.status).into());
    }
    Ok(response)
}

/// Performs a `GET` request and returns `Err` if the response status is not `200 OK`.
pub fn get_ok(client: &Client, url: &str) -> Result<Response, AccountError> {
    get_expect(client, url, StatusCode::Ok)
}

/// Performs a `POST` request and returns `Err` if the response status is not `200 OK`.
pub fn post_ok(
    client: &Client,
    url: &str,
    body_pairs: &[(&str, &str)]
) -> Result<Response, AccountError> {
    post_expect(client, url, body_pairs, StatusCode::Ok)
}

/// Parses the body of a response as HTML.
pub fn read_to_html(response: &mut Response) -> Result<Html, AccountError> {
    let mut body = String::new();
    try!(response.read_to_string(&mut body));
    Ok(Html::parse_document(&body))
}

/// Selects the first element matching a selector, or returns `Err`.
pub fn select_one<'a>(
    url: &str,
    html: &'a Html,
    selector: &str
) -> Result<NodeRef<'a>, AccountError> {
    html.select(&Selector::parse(selector).unwrap())
        .next()
        .ok_or_else(|| MarkupError::missing_element(url, selector).into())
}

/// Selects the first element matching a selector and returns the value of one of its attributes,
/// or returns `Err`.
pub fn select_attr<'a>(
    url: &str,
    html: &'a Html,
    selector: &str,
    attr: &str
) -> Result<&'a str, AccountError> {
    let node = try!(select_one(url, html, selector));
    node.value()
        .as_element()
        .and_then(|e| e.attr(attr))
        .ok_or_else(|| MarkupError::missing_attr(url, selector, attr).into())
}

/// Finds an inbox message, or returns `Err`.
pub fn inbox_find<A: Authenticator<Installed>>(
    inbox: &Inbox<A>,
    q: &str
) -> Result<Message, AccountError> {
    match try!(inbox.find(q)) {
        Some(m) => Ok(m),
        None => Err(MessageError::Missing(String::from(q)).into()),
    }
}

/// Finds a message part and decodes it.
pub fn decode_part(message: &Message, mime_type: &str) -> Result<String, AccountError> {
    let data = message.find_part_by_type(mime_type)
        .and_then(|p| p.body.as_ref())
        .and_then(|b| b.data.as_ref());
    let data = match data {
        Some(d) => d,
        None => return Err(MessageError::MissingPart(String::from(mime_type)).into()),
    };
    let bytes = try!(data.from_base64());
    let string = try!(String::from_utf8(bytes));
    Ok(string)
}

/// Finds regex captures in a string.
pub fn regex_captures<'t>(regex: &str, input: &'t str) -> Result<Captures<'t>, AccountError> {
    let re = Regex::new(regex).unwrap();
    match re.captures(input) {
        Some(c) => Ok(c),
        None => Err(MessageError::Regex(String::from(regex)).into()),
    }
}
