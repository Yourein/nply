use webbrowser;
use std::error::Error;

#[allow(dead_code)]
const URL_ENCODED_REDIRECT_URI: &str = "http%3A%2F%2Flocalhost%3A8080%2Fcallback";
#[allow(dead_code)]
const UNENCODED_REDIRECT_URI: &str = "http://localhost:8080/callback";


async fn throw_url(carg: &str) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    match webbrowser::open(carg) {
        Ok(_) => Ok(()),
        Err(_) => panic!()
    }
}

#[allow(non_snake_case)]
#[allow(dead_code)]
pub mod OAuth1{
    use super::throw_url;
    use std::error::Error;
    use std::{str, io};
    use std::io::Write;
    use reqwest;
    use url::Url;
    use chrono::Local;
    use urlencoding::encode as p_encode;
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;
    use base64::encode as b64_encode;
    use serde::{Deserialize};

    const API_REQUEST_TOKEN_URL: &str = "https://api.twitter.com/oauth/request_token";
    const API_AUTHORIZE_URL: &str = "https://api.twitter.com/oauth/authorize";
    const API_ACCESS_TOKEN_URL: &str = "https://api.twitter.com/oauth/access_token";

    #[derive(Debug)]
    pub struct OAuthHeader {
        encryption_key: String,
        oauth_consumer_key: String,
        oauth_nonce: String,
        pub oauth_signature: String,
        oauth_signature_method: String,
        oauth_timestamp: String,
        oauth_version: String
    }

    impl OAuthHeader {
        pub fn new<'a> (
            api_key: &'a str,
            api_secret: &'a str,
        ) -> OAuthHeader {
            let nonce = Self::create_oauth_nonce();
            let current_timestamp = Local::now().timestamp().to_string();
            let crypt_key = format!{"{}&", p_encode(api_secret)};
            let signature = Self::create_signature(api_key, &nonce, "HMAC-SHA1", &current_timestamp, &crypt_key);
            
            OAuthHeader {
                encryption_key: crypt_key,
                oauth_consumer_key: api_key.to_string(),
                oauth_nonce: nonce,
                oauth_signature: signature,
                oauth_signature_method: "HMAC-SHA1".to_string(),
                oauth_timestamp: current_timestamp,
                oauth_version: "1.0".to_string()
            }
        }

        pub fn generate_oauth_header_string(&self) -> String {
            format!{
                r#"OAuth oauth_consumer_key="{}", oauth_nonce="{}", oauth_signature="{}", oauth_signature_method="HMAC-SHA1", oauth_timestamp="{}", oauth_version="1.0", oauth_callback="oob""#,
                &self.oauth_consumer_key,
                &self.oauth_nonce,
                p_encode(&self.oauth_signature),
                &self.oauth_timestamp
            }
        }

        fn create_oauth_nonce() -> String {
            let rand_str: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(32)
                .map(char::from)
                .collect();

            let b64_encoded_str: String = b64_encode(rand_str);
            let res = b64_encoded_str.replace(&['+', '/', '='], "");

            res
        }

        fn create_signature(
            api_key: &str,
            nonce: &str,
            signature_method: &str,
            timestamp: &str,
            crypt_key: &str
        ) -> String {
            let parameter_string = format!{
                "oauth_callback=oob&{}&{}&{}&{}&oauth_version=1.0",
                format!{"oauth_consumer_key={}",     api_key},
                format!{"oauth_nonce={}",            nonce},
                format!{"oauth_signature_method={}", signature_method},
                format!{"oauth_timestamp={}",        timestamp},
            };

            let base = format!{"POST&{}&{}", p_encode(&API_REQUEST_TOKEN_URL), p_encode(&parameter_string)};

            let hash = hmacsha1::hmac_sha1(crypt_key.as_bytes(), base.as_bytes());
            b64_encode(&hash).to_string()
        }
    }

    #[derive(Deserialize, Debug)]
    pub struct RequestTokenResponse {
        pub oauth_token: String,
        pub oauth_token_secret: String,
        pub oauth_callback_confirmed: bool
    }

    #[derive(Deserialize, Debug)]
    pub struct TokenResponse {
        oauth_token: String,
        oauth_token_secret: String
    }

    pub struct ApiCredential {
        api_key: String,
        api_secret: String,
        request_oauth_token: Option<String>,
        request_oauth_token_secret: Option<String>,
        oauth_token: Option<String>,
        oauth_token_secret: Option<String>,
        oauth_verifier: Option<String>
    }

    impl ApiCredential {
        pub fn new<'a>(
            api_key: &'a str,
            api_secret: &'a str
        ) -> ApiCredential {
            ApiCredential {
                api_key: api_key.to_string(),
                api_secret: api_secret.to_string(),
                request_oauth_token: None,
                request_oauth_token_secret: None,
                oauth_token: None,
                oauth_token_secret: None,
                oauth_verifier: None
            }
        }

        async fn request_token(&self) -> Result<RequestTokenResponse, Box<dyn Error + Send + Sync + 'static>> {
            let header_factory = OAuthHeader::new(&self.api_key, &self.api_secret);
            let request_url = format!{"{}?oauth_callback=oob", API_REQUEST_TOKEN_URL};

            let request_tokens = reqwest::Client::new().post(request_url)
                .header("Authorization", header_factory.generate_oauth_header_string())
                .send().await?;

            let params = request_tokens.text().await?;
            let parse_url = Url::parse(&(format!{"http://localhost?{}", params})).unwrap();

            let mut token = "".to_string();
            let mut token_secret = "".to_string();
            let mut confirmed = false;
            for (key, value) in parse_url.query_pairs() {
                match key {
                    std::borrow::Cow::Borrowed("oauth_token") => { token = value.to_string(); },
                    std::borrow::Cow::Borrowed("oauth_token_secret") => { token_secret = value.to_string(); },
                    std::borrow::Cow::Borrowed("oauth_callback_confirmed") => { confirmed = value == "true"; },
                    std::borrow::Cow::Owned(_) => continue,
                    std::borrow::Cow::Borrowed(_) => continue
                }
            }

            assert_ne!(token, "".to_string());
            assert_ne!(token_secret, "".to_string());
            assert_ne!(confirmed, false);

            Ok(RequestTokenResponse {
                oauth_token: token,
                oauth_token_secret: token_secret,
                oauth_callback_confirmed: confirmed
            })
        }

        async fn request_pin(&self, oauth_token: &str) -> Result<String, String> {
            let url_with_param = format!{"{}?oauth_token={}", API_AUTHORIZE_URL, p_encode(oauth_token)};

            if throw_url(&url_with_param).await.is_err() {
                panic!();
            }

            print!{"Please input the PIN number: "};
            io::stdout().flush().unwrap();
            let mut raw_pin = String::new();
            io::stdin().read_line(&mut raw_pin).except("stdin Error");
            let pin = raw_pin.replace("\n", "");

            let validater = pin.clone();
            let is_valid_pin = validater.len() == 7 && !validater.chars().map(|c| c.is_numeric()).collect::<Vec<bool>>().contains(&false);

            return if is_valid_pin { Ok(pin) } else { Err("Invalid PIN".to_string()) } 
        }

        async fn request_access_token(&self, unverified_token: &str, verifier: &str) -> Result<TokenResponse, String> {
            let request_url = format!{"{}?oauth_token={}&oauth_verifier={}", API_ACCESS_TOKEN_URL, unverified_token, verifier};

            let request_res = reqwest::Client::new().post(request_url)
                .send().await.unwrap();

            let params = request_res.text().await.unwrap();
            let parse_url = Url::parse(&(format!{"http://localhost?{}", params})).unwrap();
            let mut access_token = String::new();
            let mut access_token_secret = String::new();
            for (key, value) in parse_url.query_pairs() {
                match key {
                    std::borrow::Cow::Borrowed("oauth_token") => { access_token = value.to_string(); },
                    std::borrow::Cow::Borrowed("oauth_token_secret") => { access_token_secret = value.to_string(); },
                    std::borrow::Cow::Borrowed(_) => continue,
                    std::borrow::Cow::Owned(_) => continue,
                }
            }

            assert_ne!(access_token, String::new());
            assert_ne!(access_token_secret, String::new());

            Ok(TokenResponse {
                oauth_token: access_token,
                oauth_token_secret: access_token_secret
            })
        }

        pub async fn perform_auth(&mut self) -> Result<(), String> {
            println!{"\x1b[1;97mperform_auth:\x1b[0;97m Requesting unverified oauth_token..."};
            
            match self.request_token().await {
                Ok(tokens) => {
                    self.request_oauth_token = Some(tokens.oauth_token);
                    self.request_oauth_token_secret = Some(tokens.oauth_token_secret);
                },
                Err(e) => {
                    println!{"{}", e}
                    panic!{}
                }
            }

            println!{"\x1b[1;97mperform_auth:\x1b[0;97m \x1b[1;32mReceived tokens.\x1b[0;97m"}
            println!{"\x1b[1;97mperform_auth:\x1b[0;97m Opening your web browser..."}

            match self.request_pin(&self.request_oauth_token.as_ref().unwrap()).await {
                Ok(pin) => {
                    self.oauth_verifier = Some(pin);
                }
                Err(e) => {
                    return Err(e);
                }
            }

            println!{"\x1b[1;97mperform_auth:\x1b[0;97m \x1b[1;32mPIN is valid.\x1b[0;97m"}
            println!{"\x1b[1;97mperform_auth:\x1b[0;97m Requesting oauth_token..."};

            match self.request_access_token(
                &self.request_oauth_token.as_ref().unwrap(),
                &self.oauth_verifier.as_ref().unwrap()
            ).await {
                Ok(tokens) => {
                    self.oauth_token = Some(tokens.oauth_token);
                    self.oauth_token_secret = Some(tokens.oauth_token_secret);
                },
                Err(e) => {
                    return Err(e);
                }
            }
            
            println!{"\x1b[1;97mperform_auth:\x1b[0;97m \x1b[1;32mAPI access granted.\x1b[0;97m"}
            
            Ok(())
        }
    }
}



#[allow(non_snake_case)]
#[allow(dead_code)]
pub mod OAuth2 {
    use super::{URL_ENCODED_REDIRECT_URI, UNENCODED_REDIRECT_URI, throw_url};
    use url::Url;
    use std::io::{BufRead, BufReader, Write};
    use std::net::TcpListener;
    use std::error::Error;
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;
    use std::collections::HashMap;
    use reqwest;
    use serde::{Deserialize};
   
    const API_AUTHORIZE_URL: &str = "https://twitter.com/i/oauth2/authorize";
    const API_TOKEN_URL: &str = "https://api.twitter.com/2/oauth2/token";
    
    fn generate_random_str(len: i32) -> String {
        thread_rng().sample_iter(&Alphanumeric).take(len as usize).map(char::from).collect()
    }

    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    pub struct AuthResponse {
        token_type: String,
        expires_in: i64,
        access_token: String,
        scope: String,
        refresh_token: Option<String>
    }

    #[allow(dead_code)]
    pub struct ApiCredential<'a> {
        client_id: &'a str,
//        secret_id: &'a str,
        current_state_str: Option<String>,
        current_challenge_str: Option<String>,
        access_token: Option<String>
    }

    impl ApiCredential<'_> {
        pub fn new<'a>(
            cid: &'a str,
//            sid: &'a str
        ) -> ApiCredential<'a> {
            ApiCredential {
                client_id: cid,
//                secret_id: sid,
                current_state_str: None,
                current_challenge_str: None,
                access_token: None
            }
        }

        async fn request_auth_code(&mut self) -> Result<String, String> {
            self.current_state_str = Some(generate_random_str(20));
            self.current_challenge_str = Some(generate_random_str(64));

            let auth_url = format!{
                "{}?response_type=code&client_id={}&redirect_uri={}&scope=tweet.write%20tweet.read%20users.read&state={}&code_challenge={}&code_challenge_method=plain",
                API_AUTHORIZE_URL,
                self.client_id,
                URL_ENCODED_REDIRECT_URI,
                self.current_state_str.as_ref().unwrap(),
                self.current_challenge_str.as_ref().unwrap()
            };

            //Set TcpListener to listen API callback
            let listener = TcpListener::bind("localhost:8080").unwrap();
            
            if throw_url(&auth_url).await.is_err() {
                //Be panic if an error occured in opening web browser
                panic!();
            }

            let mut auth_code = String::new();
            let mut is_same_state = true;
            for stream in listener.incoming() {
                let mut message = "OK";

                if let Ok(mut stream) = stream {
                    let mut reader = BufReader::new(&stream);
                    let mut request_line = String::new();
                    reader.read_line(&mut request_line).unwrap();

                    println!{"reader: {}", request_line};

                    let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                    let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                    for (key, value) in url.query_pairs() {
                        match key {
                            std::borrow::Cow::Borrowed("code") => { auth_code = value.to_string(); },
                            std::borrow::Cow::Borrowed("state") => {
                                if self.current_state_str.as_ref().unwrap() != &value.to_string() {
                                    message = "Different state parameter";
                                    is_same_state = false;
                                }
                            },
                            std::borrow::Cow::Owned(_) => unreachable!(),
                            std::borrow::Cow::Borrowed(_) => unreachable!()
                            //These 2 pattern should be unreachable. I don't know.
                        }
                    }

                    let response = format!{
                        "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                        message.len(),
                        message
                    };
                    stream.write_all(response.as_bytes()).unwrap();
                    break;
                }
            }

            if is_same_state {
                Ok(auth_code)
            }
            else {
                Err("API returned different state parameter".to_string())
            }
        }

        pub async fn perform_auth(&mut self) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
            let auth_code: String = self.request_auth_code().await.unwrap();
            let mut params = HashMap::new();

            params.insert("code", auth_code.clone());
            params.insert("grant_type", "authorization_code".to_string());
            params.insert("client_id", self.client_id.to_string());
            params.insert("redirect_uri", UNENCODED_REDIRECT_URI.to_string());
            params.insert("code_verifier", self.current_challenge_str.as_ref().unwrap().clone());

            let res = reqwest::Client::new().post(API_TOKEN_URL)
                .form(&params)
                .send().await?
                .json::<AuthResponse>().await?;

            self.access_token = Some(res.access_token);
            Ok(())
        }

        pub fn get_access_token(&self) -> String {
            self.access_token.as_ref().unwrap().clone()
        }
    }
}
