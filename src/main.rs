extern crate serenity;

pub mod secret;

use serenity::client::Client;

fn main() {
    let client = Client::builder(secret::TOKEN);
}
