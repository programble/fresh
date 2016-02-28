use fresh::account::{Account, HackerNews, Lobsters};
use fresh::gmail::Inbox;
use hyper::Client as HttpClient;

use authenticator::Prompt;

pub fn reset_password(
    account_type: &str,
    account_user: &str,
    inbox: &Inbox<Prompt>,
    password: &str,
    archive: bool,
    verbose: bool,
) {
    match account_type {
        "hackernews" => account_reset(
            HackerNews { username: String::from(account_user) },
            inbox,
            password,
            archive,
            verbose,
        ),
        "lobsters" => account_reset(
            Lobsters { email: String::from(account_user) },
            inbox,
            password,
            archive,
            verbose,
        ),
        _ => unreachable!(),
    }
}

fn account_reset<A: Account>(
    account: A,
    inbox: &Inbox<Prompt>,
    password: &str,
    archive: bool,
    verbose: bool,
) {
    let mut http = HttpClient::default();
    http.set_redirect_policy(account.redirect_policy());

    if verbose { println!("Initiating reset..."); }
    account.initiate_reset(&http).unwrap();

    if verbose { println!("Finding message..."); }
    let message = account.find_message(inbox).unwrap();
    if verbose { println!("Found message: {:?}", message); }

    if verbose { println!("Parsing message..."); }
    let key = account.parse_message(&message).unwrap();
    if verbose { println!("Reset key: {:?}", key); }

    if verbose { println!("Setting password..."); }
    account.set_password(&http, &key, password).unwrap();

    if archive {
        if verbose { println!("Archiving message..."); }
        inbox.archive(&message).unwrap();
    }

    println!("Login URL: {}", account.login_url());
    println!("Password: {}", password);
}
