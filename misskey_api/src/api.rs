use crate::types::*;
use crate::types::Responses::*;
use interface::PostAPI;
use async_trait::async_trait;
use bytes::Bytes;
use reqwest;

#[allow(dead_code)]
pub struct MisskeyApi {
    access_code: String,
    base_url: String
}

impl MisskeyApi {
    pub fn new(code: String, host: String) -> Self {
        MisskeyApi {
            access_code: code,
            base_url: format!{"https://{}/api", host}
        }
    }

    pub async fn check_picture_exist(picture: &Bytes) -> Result<bool, String> {
        todo!()
    }

    fn hash_picture(picture: &Bytes) -> String {
        todo!()
    }
}

#[async_trait]
impl PostAPI for MisskeyApi {
    async fn compose_without_picture(&self, text: &str) -> Result<(), String> {
        let payload = NoteWithoutPicture {
            i: &self.access_code,
            text: text.to_string()
        };

        let res = reqwest::Client::new().post(format!{"{}/notes/create", &self.base_url})
            .header("Content-Type", "application/json")
            .json(&payload)
            .send().await;

        match res {
            Ok(r) => {
                if r.status().as_u16() == 200 {
                    Ok(())
                }
                else {
                    let e = r.json::<CommonError>().await.unwrap();
                    Err(e.message)
                }
            },
            Err(e) => {
                Err("Unknown Error Occured".to_string())
            }
        }
    }

    async fn compose_with_picture(&self, text: &str, media: &Vec<String>) -> Result<(), String> {
        todo!()
    }

    async fn upload_media(&self, picture: Bytes) -> Option<String> {
        todo!()
    }
}
