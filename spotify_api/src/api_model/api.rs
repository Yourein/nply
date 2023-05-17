use crate::api_model::credentials;
use crate::api_model::responses;
use reqwest;

#[allow(dead_code)]
const CURRENT_PLAYING_API: &str = "https://api.spotify.com/v1/me/player/currently-playing";

#[allow(dead_code)]
pub struct Api<'a> {
    auth: credentials::Code::ApiCredential<'a>
}

#[allow(dead_code)]
impl Api<'_> {
    pub async fn new<'a>(
        cid: &'a str,
        sid: &'a str
    ) -> Result<Api<'a>, String> {
        let mut cred = credentials::Code::ApiCredential::new(cid, sid);

        if cred.perform_auth().await.is_err() {
            return Err("Could not authenticate".to_string());
        }

        Ok(
            Api{
                auth: cred
            }
        )
    }

    pub async fn get_current_song(&mut self) -> Result<responses::CurrentlyPlaying::Responses, String> {
        if !self.auth.is_token_valid() {
            if self.auth.perform_auth().await.is_err() {
                return Err("Could not authenticate".to_string());
            }
        }

        let rawres = reqwest::Client::new().get(CURRENT_PLAYING_API)
            .header("Authorization", format!{"Bearer {}", self.auth.get_access_token()})
            .header("Content-Type", "application/json")
            .send().await;

        if rawres.is_err() {
            Err("Cannot retrieve your current playing song".to_string())
        }
        else {
            match rawres.unwrap().json::<responses::CurrentlyPlaying::Responses>().await {
                Ok(res) => {
                    Ok(res)
                }
                Err(reason) => {
                    Err(reason.to_string())
                }
            }
        }
    }
}
