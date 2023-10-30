#[allow(dead_code)]
const API_TOKEN_URL: &str = "https://accounts.spotify.com/api/token";
const API_AUTHORIZE_URL: &str = "https://accounts.spotify.com/authorize";
const URL_ENCODED_REDIRECT_URI: &str = "http%3A%2F%2Flocalhost%3A8080%2Fcallback";
const UNENCODED_REDIRECT_URI: &str = "http://localhost:8080/callback";

#[allow(non_snake_case)]
pub mod Code {
    use chrono::prelude::{DateTime, Utc};
    use chrono::Duration;
    use std::error::Error;
    use url::Url;
    use std::io::{BufRead, BufReader, Write};
    use std::net::TcpListener;
    use utils::throw_url;
    use reqwest;
    use base64::{Engine as _, engine::general_purpose};
    use serde::Deserialize;
    use std::collections::HashMap;
    
    use crate::api_model::credentials::API_TOKEN_URL;
    use crate::api_model::credentials::API_AUTHORIZE_URL;
    use crate::api_model::credentials::URL_ENCODED_REDIRECT_URI;
    use crate::api_model::credentials::UNENCODED_REDIRECT_URI;

    //-----------------------Fetching AuthCode---------------------------------

    async fn request_auth_code(cid: &str) -> Result<String, String> {
        let purl = format!{
            "{}?response_type=code&client_id={}&scope=user-read-currently-playing&redirect_uri={}",
            &API_AUTHORIZE_URL,
            cid,
            &URL_ENCODED_REDIRECT_URI
        };

        //Set TcpListener to Listen API callback
        let listener = TcpListener::bind("localhost:8080").unwrap();
       
        //Open webbrowser to show prompt for the user
        match throw_url(&purl).await {
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

                // This html closes the tab itself automatically.
                let message: String = vec![
                    "<!DOCTYPE html>",
                    "<html>",
                    "<body onload=\"open(location, '_self').close();\">",
                    "</body>",
                    "</html>"
                ].concat();
                let response = format!{"HTTP/1.1 OK\rcontent_length: {}\r\n\r\n{}", message.len(), message};
                stream.write_all(response.as_bytes()).unwrap();
                break;
            }
        }

        Ok(auth_code)
    }

    //------------------------ApiCredential---------------------------------

    //Allowing "dead_code" because we don't need scope, token_type
    #[allow(dead_code)]
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
            general_purpose::STANDARD_NO_PAD.encode(original)
        }

        pub async fn perform_auth(&mut self) 
            -> Result<(), Box<dyn Error + Send + Sync + 'static>>
        {
            let auth_code: String = request_auth_code(self.client_id).await.unwrap();
            let cred = self.make_auth_header();
            
            let mut params = HashMap::new();

            params.insert("redirect_uri", UNENCODED_REDIRECT_URI.to_string());
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

        pub fn is_token_valid(&self) -> bool {
            self.access_token != "" && Utc::now() < self.expires_at
        }

        pub fn get_access_token(&self) -> String {
            self.access_token.clone()
        }
    }
}
