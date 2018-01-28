use error;
use client;

pub fn demo_site() -> error::Result<client::Client> {
    let site = "http://demo.subsonic.org";
    let user = "guest3";
    let password = "guest";
    client::Client::new(site, user, password)
}
