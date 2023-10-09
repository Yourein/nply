use interface::PostAPI;

#[allow(dead_code)]
pub struct MisskeyApi {
    access_code: String
}

impl MisskeyApi {
    pub fn new(code: String) -> Self {
        MisskeyApi {
            access_code: code;
        }
    }

    pub async fn check_picture_exist(picture: &Bytes) -> Result<bool, String> {
        TODO!()
    }
}

#[async_trait]
impl PostAPI for MisskeyApi {
    async fn compose_without_picture(&self, text: &str) -> Result<(), String> {
        TODO!()
    }

    async fn compose_with_picture(&self, text: &str, media: &Vec<String>) -> Result<(), String> {
        TODO!()
    }

    async fn upload_media(&self, picture: Bytes) -> Option<String> {
        TODO!()
    }
}
