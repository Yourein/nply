use webbrowser;
use std::error::Error;

#[allow(dead_code)]
const URL_ENCODED_REDIRECT_URI: &str = "http%3A%2F%2Flocalhost%3A8080%2Fcallback";
#[allow(dead_code)]
const UNENCODED_REDIRECT_URI: &str = "http://localhost:8080/callback";


async fn throw_auth_url(carg: &str) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    match webbrowser::open(carg) {
        Ok(_) => Ok(()),
        Err(_) => panic!()
    }
}

#[allow(non_snake_case)]
#[allow(dead_code)]
pub mod OAuth1{
    use super::{URL_ENCODED_REDIRECT_URI, UNENCODED_REDIRECT_URI, throw_auth_url};
    use std::io::{BufRead, BufReader, Write};
    use std::net::TcpListener;
    use std::error::Error;
    use reqwest;
    use url::Url;

    const API_REQUEST_TOKEN_URL: &str = "https://api.twitter.com/oauth/request_token";

    #[allow(dead_code)]
    pub struct ApiCredential<'a> {
        api_key: &'a str,
        api_secret: &'a str,
        oauth_token: Option<String>,
        oauth_token_secret: Option<String>
    }

    impl ApiCredential<'_>{
        pub fn new<'a>(
            pub_key: &'a str,
            secret_key: &'a str
        ) -> ApiCredential<'a> {
            ApiCredential{
                api_key: pub_key,
                api_secret: secret_key,
                oauth_token: None,
                oauth_token_secret: None
            }
        }

        pub async fn request_token(&mut self) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
            let request_url = format!{
                "{}?oauth_callback={}",
                API_REQUEST_TOKEN_URL,
                URL_ENCODED_REDIRECT_URI
            };

            println!{"Here"};

            let listener = TcpListener::bind("localhost:8080").unwrap();

            let pres = reqwest::Client::new().post(request_url)
                .header("Authorization", "OAuth".to_string())
                .header("oauth_consumer_key", self.api_key.to_string())
                .send().await?
                .text().await?;

            println!{"{}", pres};

            for stream in listener.incoming() {
                let mut message = "OK";

                if let Ok(mut stream) = stream {
                    let mut reader = BufReader::new(&stream);
                    let mut request_line = String::new();
                    reader.read_line(&mut request_line).unwrap();

                    let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                    let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                    for (key, value) in url.query_pairs() {
                        match key {
                            std::borrow::Cow::Borrowed("oauth_token") => { self.oauth_token = Some(value.to_string()); },
                            std::borrow::Cow::Borrowed("oauth_token_secret") => { self.oauth_token_secret = Some(value.to_string()); },
                            std::borrow::Cow::Owned(_) => (),
                            std::borrow::Cow::Borrowed(_) => ()
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

            Ok(())
        }
    }
}



#[allow(non_snake_case)]
#[allow(dead_code)]
pub mod OAuth2 {
    use super::{URL_ENCODED_REDIRECT_URI, UNENCODED_REDIRECT_URI, throw_auth_url};
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
            
            if throw_auth_url(&auth_url).await.is_err() {
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
