use serde::{Deserialize};

#[allow(dead_code)]
const API_TOKEN_URL: &str = "https://accounts.spotify.com/api/token";

#[derive(Deserialize)]
struct AuthResponse {
    access_token: String,
    expires_in: i32
}

#[allow(dead_code)]
#[allow(non_snake_case)]
pub mod Bearer {
    use chrono::prelude::{DateTime, Utc};
    use chrono::Duration;
    use base64;
    use reqwest;
    use crate::api_model::credentials::AuthResponse;

    pub struct ApiCredential<'a> {
        client_id: &'a str,
        secret_id: &'a str,
        access_token: String,
        expires_at: DateTime<Utc>,
    }

    impl ApiCredential<'_> {
        pub fn new<'a>(
            cid: &'a str,
            sid: &'a str
        ) -> ApiCredential<'a> {
            let now = Utc::now();

            return ApiCredential {
                client_id: cid,
                secret_id: sid,
                access_token: "".to_string(),
                expires_at: now
            }
        }

        fn make_client_credentials(&self) -> String {
            let original = format!{"{}:{}", self.client_id, self.secret_id};
            base64::encode(original)
        }

        pub async fn perform_auth(&mut self) -> reqwest::Result<()> {
            let cred = self.make_client_credentials();

            let param = [("grant_type", "client_credentials")];
            let res = reqwest::Client::new().post("https://accounts.spotify.com/api/token")
                .form(&param)
                .header("Authorization", format!{"Basic {}", cred})
                .send().await?
                .json::<AuthResponse>().await?;

            self.access_token = res.access_token;
            self.expires_at = Utc::now() + Duration::seconds(res.expires_in.into());
                
            Ok(())
        }

        pub fn is_token_valid(&self) -> bool {
            self.access_token != "" && Utc::now() < self.expires_at
        }

        pub fn get_access_token(&self) -> String {
            let temp = self.access_token.clone();
            temp
        }
    }
}
