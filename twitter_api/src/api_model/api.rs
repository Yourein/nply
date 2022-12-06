use crate::api_model::credentials;
use serde::Serialize;

//Use POST to send new tweet
const MANAGE_TWEET_API: &str = "https://api.twitter.com/2/tweets";

#[allow(dead_code)]
pub struct Api<'a> {
    auth: credentials::OAuth2::ApiCredential<'a>
}


#[derive(Serialize)]
struct Tweet {
    text: String,
}


#[allow(dead_code)]
impl Api<'_> {
    pub fn new<'a>(
        cid: &'a str
    ) -> Api {
        Api {
            auth: credentials::OAuth2::ApiCredential::new(cid)
        }
    }

    pub async fn compose_new_tweet(&mut self) -> reqwest::Result<()> {
        match self.auth.perform_auth().await {
            Ok(_) => (),
            Err(_) => panic!()
        }

        let payload = Tweet {
            text: "Hello from Twitter API!".to_string()
        };

        let res = reqwest::Client::new().post(MANAGE_TWEET_API)
            .json(&payload)
            .header("Authorization", format!{"Bearer {}", self.auth.get_access_token()})
            .send().await?
            .text().await?;

        println!{"{}", res};
        Ok(())
    }
}
