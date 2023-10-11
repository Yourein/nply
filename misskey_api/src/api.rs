use crate::types::*;
use crate::types::Responses::*;
use crate::consts::ENDPOINT;
use interface::PostAPI;
use async_trait::async_trait;
use bytes::Bytes;
use chrono::Utc;
use md5;
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

    /// Check if hashed picture is already on Drive or not.
    pub(crate) async fn check_picture_exist(&self, md5: &str) -> Result<bool, String> {
        match self.find_by_hash(&md5).await {
            Ok(res) => {
                if res.len() > 0 { Ok(true) } else { Ok(false) }
            },
            Err(e) => {
                Err(e)
            }
        }
    }

    /// Find file by its md5 hash.
    /// If nothing match, FileSearchResult.result.len() will be 0 (empty vec)
    pub(crate) async fn find_by_hash(&self, md5: &str) -> Result<Vec<DriveFile>, String> {
        let payload = Md5Container {
            i: &self.access_code,
            md5: md5.to_string()
        };

        let res = reqwest::Client::new()
            .post(self.get_endpoint_url(ENDPOINT::drive::files::find_by_hash))
            .json(&payload)
            .send().await;

        match res {
            Ok(r) => {
                if r.status().as_u16() == 200 {
                    let search_result = r.json::<Vec<DriveFile>>().await.unwrap();
                    Ok(search_result)
                }
                else {
                    let e = r.json::<CommonError>().await.unwrap();
                    Err(e.message.unwrap())
                }
            },
            Err(_) => {
                Err("Unknown Error Occured".to_string())
            }
        }
    }

    fn hash_picture(&self, picture: &Bytes) -> String {
        format!{"{:x}", md5::compute(picture)}
    }

    pub(crate) fn create_image_part(&self, picture: Bytes, name: String) -> multipart::Part {
        let mime_type = self.get_image_mime_type(&picture);
        let pic_array: Vec<u8> = picture.try_into().unwrap();

        multipart::Part::bytes(pic_array)
            .mime_str(mime_type).unwrap()
            .file_name(name)
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
            Err(_e) => {
                Err("Unknown Error Occured".to_string())
            }
        }
    }

    async fn compose_with_picture(&self, text: &str, media: &Vec<String>) -> Result<(), String> {
        todo!()
    }

    async fn upload_media(&self, picture: Bytes) -> Option<String> {
        let file_hash = self.hash_picture(&picture);
        let already_exist = self.check_picture_exist(&file_hash).await;

        if already_exist.is_ok() && already_exist == Ok(false) {
            let payload = multipart::Form::new()
                .text("i", format!{"{}", &self.access_code})
                .part("file", self.create_image_part(picture, format!{"{}", Utc::now().timestamp()}));

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
                },
                Err(_) => {
                    None
                }
            }
        }
        else {
            match self.find_by_hash(&file_hash).await {
                Ok(r) => {
                    Some(r[0].id.clone())
                },
                Err(_) => {
                    None
                }
            }
        }
    }
}
