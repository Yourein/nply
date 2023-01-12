use crate::api_model::oauth1_session::OAuth1Session;
use std::collections::BTreeMap;

#[allow(dead_code)]
const MANAGE_TWEET_API: &str = "https://api.twitter.com/2/tweets";

#[allow(dead_code)]
pub struct Api {
    session: OAuth1Session
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

    pub async fn compose_new_tweet(&self, text: &str) -> Result<reqwest::Response, reqwest::Error> {
        let request_url = "https://api.twitter.com/1.1/statuses/update.json";
        let url = url::Url::parse(&request_url).unwrap();
        
        let mut request_param = BTreeMap::new();
        let extra_oauth_param = BTreeMap::new();
        request_param.insert("status".to_string(), text.to_owned().to_string());

        let body = request_param
            .iter()
            .map(|(k, v)| format!{"{}={}", k, urlencoding::encode(v)})
            .collect::<Vec<String>>()
            .join("&");

        let client = reqwest::Client::new();
        let req = client.post(request_url).body(body);

        self.session.post(
            req,
            &url,
            &request_param,
            &extra_oauth_param
        ).await
    }
}
