#[allow(dead_code)]
const API_TOKEN_URL: &str = "https://accounts.spotify.com/api/token";
const API_AUTHORIZE_URL: &str = "https://accounts.spotify.com/authorize";
const REDIRECT_URI: &str = "http%3A%2F%2Flocalhost%3A8080%2Fcallback";

#[allow(dead_code)]
#[allow(non_snake_case)]
pub mod Bearer {
    #[allow(unused_imports)]
    use chrono::prelude::{DateTime, Utc};
    use chrono::Duration;
    use base64;
    use reqwest;
    use serde::{Deserialize};
    use crate::api_model::credentials::API_TOKEN_URL;

    #[derive(Deserialize, Debug)]
    struct AuthResponse {
        access_token: String,
        token_type: String,
        expires_in: i64
    }

    pub struct ApiCredential<'a> {
        client_id: &'a str,
        secret_id: &'a str,
        access_token: String,
        expires_at: DateTime<Utc>,
    }


    #[allow(dead_code)]
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

        fn make_auth_header(&self) -> String {
            let original = format!{"{}:{}", self.client_id, self.secret_id};
            base64::encode(original)
        }

        pub async fn perform_auth(&mut self) -> reqwest::Result<()> {
            let cred = self.make_auth_header();

            let param = [("grant_type", "client_credentials")];
            let res = reqwest::Client::new().post(API_TOKEN_URL)
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

#[allow(dead_code)]
#[allow(non_snake_case)]
pub mod Code {
    use chrono::prelude::{DateTime, Utc};
    use chrono::Duration;
    use std::error::Error;
    use url::Url;
    use std::io::{BufRead, BufReader, Write};
    use std::net::TcpListener;
    use webbrowser;
    use reqwest;
    use base64;
    use serde::Deserialize;
    use std::collections::HashMap;
    
    use crate::api_model::credentials::API_TOKEN_URL;
    use crate::api_model::credentials::API_AUTHORIZE_URL;
    use crate::api_model::credentials::REDIRECT_URI;

    //-----------------------Fetching AuthCode---------------------------------

    async fn throw_auth_url(carg: &str) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        match webbrowser::open(carg) {
            Ok(_) => Ok(()),
            Err(_) => panic!() 
        }
    }

    async fn request_auth_code(cid: &str) -> Result<String, Box<dyn Error + Send + Sync + 'static>> {
        let purl = format!{
            "{}?response_type=code&client_id={}&scope=user-read-currently-playing&redirect_uri={}",
            &API_AUTHORIZE_URL,
            cid,
            &REDIRECT_URI
        };

        //Set TcpListener to Listen API callback
        let listener = TcpListener::bind("localhost:8080").unwrap();
       
        //Open webbrowser to show prompt for the user
        match throw_auth_url(&purl).await {
            Ok(_) => (),
            Err(e) => return Err(e)
        }

        let mut auth_code = String::new();
        for stream in listener.incoming() {
            if let Ok(mut stream) = stream {
                {
                    let mut reader = BufReader::new(&stream);
                    let mut request_line = String::new();
                    reader.read_line(&mut request_line).unwrap();

                    /*
                     * GET /callback?code=*** HTTP/1.1
                     *                 ^
                     *                 |
                     *     fetch parameter "code" from HTTP GET request
                     */
                    let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                    let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                    let (_, value) = url.query_pairs()
                                        .find(|pair| {
                                            let &(ref key, _) = pair;
                                            key == "code"
                                        }).unwrap();
                    
                    auth_code = value.to_string();
                }
                
                let message = "OK";
                let response = format!{"HTTP/1.1 OK\rcontent_length: {}\r\n\r\n{}", message.len(), message};
                stream.write_all(response.as_bytes()).unwrap();
                break;
            }
        }

        Ok(auth_code)
    }

    //------------------------ApiCredential---------------------------------
    
    #[derive(Deserialize, Debug)]
    struct AuthResponse {
        access_token: String,
        token_type: String,
        scope: String,
        expires_in: i64,
        refresh_token: String
    }

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
            ApiCredential {
                client_id: cid,
                secret_id: sid,
                access_token: "".to_string(),
                expires_at: Utc::now(),
            }
        }

        fn make_auth_header(&self) -> String {
            let original = format!{"{}:{}", self.client_id, self.secret_id};
            base64::encode(original)
        }

        pub async fn perform_auth(&mut self) 
            -> Result<(), Box<dyn Error + Send + Sync + 'static>>
        {
            let auth_code: String = request_auth_code(self.client_id).await.unwrap();
            let cred = self.make_auth_header();
            
            let mut params = HashMap::new();

            //You MUST insert redirect_uri without URL-encode
            params.insert("redirect_uri", "http://localhost:8080/callback".to_string());
            params.insert("grant_type", "authorization_code".to_string());
            params.insert("code", auth_code.clone());
            
            let res = reqwest::Client::new().post(API_TOKEN_URL)
                .form(&params)
                .header("Authorization", "Basic ".to_string() + &cred)
                .send().await?
                .json::<AuthResponse>().await?;
           
            self.access_token = res.access_token;
            self.expires_at = Utc::now() + Duration::seconds(res.expires_in.into());

            Ok(())
        }

        pub fn get_access_token(&self) -> String {
            println!{"token: {}", &self.access_token};
            self.access_token.clone()
        }
    }
}
