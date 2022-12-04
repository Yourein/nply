use crate::api_model::credentials::Bearer::ApiCredential;
use reqwest;

const CURRENT_PLAYING_API: &str = "https://api.spotify.com/v1/me/player/currently-playing";

pub struct Api<'a> {
    auth: ApiCredential<'a>,
}

impl Api<'_> {
    pub fn new<'a>(
        cid: &'a str,
        sid: &'a str
    ) -> Api<'a> {
        let auth_instance = ApiCredential::new(cid, sid);
       
        Api{
            auth: auth_instance
        }
    }

    pub async fn get_current_song(&mut self) -> reqwest::Result<String> {
        if !self.auth.is_token_valid() {
            match self.auth.perform_auth().await {
                Ok(_) => (),
                Err(_) => panic!()
            }
        }

        let res = reqwest::Client::new().get(CURRENT_PLAYING_API)
            .header("Authorization", format!{"Bearer {}", self.auth.get_access_token()})
            .header("Content-Type", "application/json")
            .send().await?
            .text().await?;

        Ok(res)
    }
}
