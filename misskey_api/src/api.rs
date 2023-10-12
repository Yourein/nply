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

    /// Find file by its md5 hash.
    /// If nothing matches, FileSearchResult.result.len() will be 0 (empty vec)
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
        let payload = NoteWithPicture {
            i: &self.access_code,
            text: text.to_string(),
            mediaIds: media.to_vec()
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
            Err(_) => {
                Err("Unknown Error Occured".to_string())
            }
        }
    }

    async fn upload_media(&self, picture: Bytes) -> Option<String> {
        let file_hash = self.hash_picture(&picture);
        let search_result = self.find_by_hash(&file_hash).await;

        match search_result {
            Ok(sr) => {
                if sr.len() == 0 {
                    // Using format! at .text() to avoid problems about borrowing/move
                    let payload = multipart::Form::new()
                        .text("i", format!{"{}", &self.access_code})
                        .part("file", self.create_image_part(picture, format!{"{}", Utc::now().timestamp()}));

                    let res = reqwest::Client::new()
                        .post(self.get_endpoint_url(ENDPOINT::drive::files::create))
                        .multipart(payload)
                        .send().await;

                    // Ignoring errors 4** and 5**
                    // Put a condition of res.is_ok() && status != 200 to debug
                    if res.is_ok() && res.as_ref().ok()?.status().as_u16() == 200 {
                        let f = res.unwrap().json::<DriveFile>().await.unwrap();
                        Some(f.id)
                    }
                    else {
                        None
                    }
                }
                else {
                    Some(sr[0].id.clone())
                }
            },
            Err(_) => {
                None
            }
        }
    }
}
