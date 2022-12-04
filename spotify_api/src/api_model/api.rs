use crate::api_model::credentials;
use reqwest;

#[allow(dead_code)]
const CURRENT_PLAYING_API: &str = "https://api.spotify.com/v1/me/player/currently-playing";

#[allow(dead_code)]
pub struct Api<'a> {
    Auth: credentials::Code::ApiCredential<'a>
}

#[allow(dead_code)]
impl Api<'_> {
    pub fn new<'a>(
        cid: &'a str,
        sid: &'a str
    ) -> Api<'a> {
        Api{
            Auth: credentials::Code::ApiCredential::new(cid, sid)
        }
    }

    //TODO: Create some model for the API response
    pub async fn get_current_song(&mut self) -> reqwest::Result<String> {
        if self.Auth.perform_auth().await.is_err() {
            panic!();
        }

        let res = reqwest::Client::new().get(CURRENT_PLAYING_API)
            .header("Authorization", format!{"Bearer {}", self.Auth.get_access_token()})
            .header("Content-Type", "application/json")
            .send().await?
            .text().await?;

        Ok(res)
    }
}
