use crate::api_model::oauth1_session::OAuth1Session;
use serde::Serialize;

#[allow(dead_code)]
const MANAGE_TWEET_API: &str = "https://api.twitter.com/2/tweets";

#[allow(dead_code)]
pub struct Api {
    session: OAuth1Session
}


#[derive(Serialize)]
struct Tweet {
    text: String,
}


#[allow(dead_code)]
impl Api {
    pub async fn new<'a>(
        apikey: &'a str,
        apisecret: &'a str
    ) -> Api {
        Api {
            session: OAuth1Session::new(
                apikey.to_string(),
                apisecret.to_string()
            ).await
        }
    }

    pub async fn compose_new_tweet(&mut self) -> reqwest::Result<()> {
        todo!()
    }
}
