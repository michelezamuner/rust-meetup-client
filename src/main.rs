#[macro_use]
extern crate log;
extern crate env_logger;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod response;
mod meetup_client;

use std::env;
use meetup_client::MeetupClient;

fn main() {

    match env_logger::init() {
        Ok(_) => debug!("logging started."),
        Err(err) => error!("error starting logger: {}", err),
    };

    let meetup_url = env::var("MEETUP_URL").expect("MEETUP_URL was not found.");
    let member_id = env::var("MEMBER_ID").expect("MEMBER_ID was not found.");
    let token = env::var("MEETUP_API_KEY").expect("MEETUP_API_KEY was not found.");
    let client = MeetupClient::new(meetup_url.as_str(), token.as_str());
    // https://www.meetup.com/meetup_api/docs/activity/
    let resp_data = client.get_activity(member_id.as_str());
    println!("{:?}", resp_data);
    // println!("{:?}", client.url);
}
