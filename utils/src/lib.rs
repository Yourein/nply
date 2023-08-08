use webbrowser;

pub async fn throw_url(carg: &str) -> Result<(), String> {
    match webbrowser::open(carg) {
        Ok(_) => Ok(()),
        Err(_) => {
            let reason = format!{"Could not open the URL: {} in your browser", carg};
            Err(reason)
        }
    }
}

#[allow(non_snake_case)]
#[allow(dead_code)]
pub mod MockPostAPI {
    use async_trait::async_trait;
    use interface::PostAPI;
    use bytes::Bytes;

    pub struct SuccessPostAPI {
        api_key: String,
        api_secret: String
    }

    impl SuccessPostAPI {
        fn new(api_key: String, api_secret: String) -> Self {
            SuccessPostAPI {
                api_key: api_key.clone(),
                api_secret: api_secret.clone()
            }
        }
    }

    #[async_trait]
    impl PostAPI for SuccessPostAPI {
        async fn upload_media(&self, _picture: Bytes) -> Option<String> {
            Some("hoge".to_string())
        }

        async fn compose_without_picture(&self, _text: &str) -> Result<(), String> {
            Ok(())
        }

        async fn compose_with_picture(&self, _text: &str, _media: &Vec<String>) -> Result<(), String> {
            Ok(())
        }
    }
}
