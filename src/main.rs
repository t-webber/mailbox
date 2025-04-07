use std::env::var;

use dotenv::dotenv;

#[derive(Debug)]
struct Credentials {
    domain: String,
    email: String,
    password: String,
}

impl Credentials {
    fn load() -> Option<Self> {
        dotenv().ok()?;
        Some(Self {
            domain: var("DOMAIN").ok()?,
            email: var("EMAIL").ok()?,
            password: var("PASSWORD").ok()?,
        })
    }
}

fn main() {
    dbg!(Credentials::load());
}
