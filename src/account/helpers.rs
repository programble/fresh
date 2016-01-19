//! Account helpers.

use std::io::Read;

use hyper::client::{Client, Response};
use hyper::header::ContentType;
use hyper::status::StatusCode;
use scraper::{Html, Selector, NodeRef};
use url::form_urlencoded;

use super::error::{AccountError, StatusError, MarkupError};

/// Performs a `GET` request and returns `Err` if the response status is not `200 OK`.
pub fn get_ok(client: &Client, url: &str) -> Result<Response, AccountError> {
    let request = client.get(url);
    let response = try!(request.send());
    if response.status != StatusCode::Ok {
        return Err(StatusError::new(url, response.status).into());
    }
    Ok(response)
}

/// Performs a `POST` request and returns `Err` if the response status is not `200 OK`.
pub fn post_ok(
    client: &Client,
    url: &str,
    body_pairs: &[(&str, &str)]
) -> Result<Response, AccountError> {
    let body = form_urlencoded::serialize(body_pairs);
    let request = client.post(url)
        .header(ContentType::form_url_encoded())
        .body(&body);
    let response = try!(request.send());
    if response.status != StatusCode::Ok {
        return Err(StatusError::new(url, response.status).into());
    }
    Ok(response)
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
