use crate::types::*;
use crate::types::Responses::*;
use crate::consts::ENDPOINT;
use interface::PostAPI;
use async_trait::async_trait;
use bytes::Bytes;
use reqwest;
use reqwest::multipart;

#[allow(dead_code)]
pub struct MisskeyApi {
    access_code: String,
    host: String
}

impl MisskeyApi {
    pub fn new(code: String, host: String) -> Self {
        MisskeyApi {
            access_code: code,
            host: host
        }
    }

    fn hash_picture(&self, picture: &Bytes) -> String {
        todo!()
    }

    /// Classify an image to image/jpeg or image/png
    /// DO NOT input an image that is neither jpeg nor png
    /// 
    /// This function is reading 2 bytes from start of the image.
    /// For detail, please read https://qiita.com/kouheiszk/items/17485ccb902e8190923b#png%E3%81%A7%E3%81%82%E3%82%8B%E3%81%93%E3%81%A8
    pub(crate) fn get_image_mime_type(&self, picture: &Bytes) -> &'static str {
        if picture[0..2] == [255, 216] {
            "image/jpeg"
        }
        else {
            "image/png"
        }
    }

    pub(crate) fn get_endpoint_url(&self, endpoint: &str) -> String {
        format!{"https://{}/api/{}", &self.host, endpoint}
    }
}

#[async_trait]
impl PostAPI for MisskeyApi {
    async fn compose_without_picture(&self, text: &str) -> Result<(), String> {
        let payload = NoteWithoutPicture {
            i: &self.access_code,
            text: text.to_string()
        };

        let res = reqwest::Client::new()
            .post(self.get_endpoint_url(ENDPOINT::notes::create))
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
                    Err(e.message.unwrap())
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
        let pic_array: Vec<u8> = picture.try_into().unwrap();
        let part_picture = multipart::Part::bytes(pic_array)
            .mime_str("image/jpeg").unwrap()
            .file_name("hoge.jpg");

        let payload = multipart::Form::new()
            .text("i", format!{"{}", &self.access_code})
            .part("file", part_picture);

        let res = reqwest::Client::new()
            .post(self.get_endpoint_url(ENDPOINT::drive::files::create))
            .multipart(payload)
            .send().await;

        match res {
            Ok(r) => {
                if r.status().as_u16() == 200 {
                    let apires = r.json::<DriveFile>().await.unwrap();
                    Some(apires.id)
                }
                else {
                    None
                }
            }
            Err(_) => {
                None
            }
        }
    }
}
