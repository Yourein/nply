use crate::api_model::oauth1_session::OAuth1Session;
use std::collections::BTreeMap;
use reqwest::multipart;
use bytes::Bytes;
use serde::Deserialize;

#[allow(dead_code)]
pub struct Api {
    session: OAuth1Session
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ImageResponse {
    image_type: String,
    w: i64,
    h: i64
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct PostMediaResponse {
    media_id: i128,
    media_id_string: String,
    media_key: Option<String>,
    size: i64,
    expires_after_secs: i64,
    image: ImageResponse
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

    pub async fn upload_picture(&self, picture: Bytes) -> Option<String> {
        let request_url = "https://upload.twitter.com/1.1/media/upload.json";
        let url = url::Url::parse(&request_url).unwrap();
        
        let picary: Vec<u8> = picture.to_owned().try_into().unwrap();
        let picture_part = multipart::Part::bytes(picary);
        let form = multipart::Form::new()
            .part("media", picture_part);
        
        let response = self.session.post_multipart(form, &url).await;

        match response {
            Ok(r) => {
                let content: PostMediaResponse = r.json().await.unwrap();
                let media_id = content.media_id_string.clone();
                Some(media_id)
            },
            Err(_) => {
                None
            }
        }
    }
}
