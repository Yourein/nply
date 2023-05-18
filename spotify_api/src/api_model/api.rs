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

    pub async fn fetch_current_song(&mut self) -> Result<responses::CurrentlyPlaying::Responses, String> {
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

    pub fn parse_current_song_result(
        &self,
        api_res: responses::CurrentlyPlaying::Responses
    ) -> responses::CurrentSong {
        let song_title = &api_res.item.name;
        let song_uri = &api_res.item.uri.strip_prefix("spotify:track:").unwrap();
        let track_artists = &api_res.item.artists.iter()
                .map(|x| { x.name.to_string() })
                .collect::<Vec<String>>();

        let album_title = &api_res.item.album.name;
        let album_artists = &api_res.item.album.artists.iter()
                .map(|x| { x.name.to_string() })
                .collect::<Vec<String>>();

        let album_art_url = &api_res.item.album.images[0].url;

        responses::CurrentSong {
            song_title   : song_title.to_string(),
            song_uri     : song_uri.to_string(),
            track_artists: track_artists.to_vec(),
            album_title  : album_title.to_string(),
            album_artists: album_artists.to_vec(),
            album_art_url: album_art_url.to_string()
        }
    }
}
