use crate::api_model::oauth1_header::OAuthHeader;

use std::collections::BTreeMap;
use reqwest;
use std::str;
use std::error::Error;
use url::Url;
use std::io;
use std::io::Write;

const API_REQUEST_TOKEN_URL: &str = "https://api.twitter.com/oauth/request_token";
const API_AUTHORIZE_URL: &str = "https://api.twitter.com/oauth/authorize";
const API_ACCESS_TOKEN_URL: &str = "https://api.twitter.com/oauth/access_token";

#[allow(dead_code)]
pub struct OAuth1Session {
    api_key: String,
    api_secret: String,
    oauth_token: Option<String>,
    oauth_token_secret: Option<String>
}

impl OAuth1Session {
    pub async fn new(apikey: String, apisecret: String) -> OAuth1Session {
        let token_result = Self::perform_auth(&apikey, &apisecret);

        let mut token = None;
        let mut token_secret = None;
        match token_result.await {
            Ok (res) => {
                token = Some(res.0);
                token_secret = Some(res.1);
            }
            _ => {}
        }

        OAuth1Session {
            api_key: apikey,
            api_secret: apisecret,
            oauth_token: token,
            oauth_token_secret: token_secret
        }
    }

    async fn request_token(apikey: &str, apisecret: &str) -> Result<(String, String), Box<dyn Error + Send + Sync + 'static>> {
        //パラメータを用意
        let request_params = BTreeMap::new();
        let mut extra_oauth_params = BTreeMap::new();
        extra_oauth_params.insert("oauth_callback".to_string(), "oob".to_string());
        let request_url = format!{"{}?oauth_callback=oob", API_REQUEST_TOKEN_URL};
       
        //headerを作る
        let headers = OAuthHeader::new(
            apikey,
            apisecret,
            "POST",
            &Url::parse(&request_url).unwrap(),
            &request_params,
            &extra_oauth_params
        );

        //tokenをリクエスト
        let response = reqwest::Client::new().post(request_url)
            .header("Authorization", headers.header)
            .send().await?
            .text().await?;

        let mut token = "".to_string();
        let mut token_secret = "".to_string();

        for (key, value) in Url::parse(&(format!{"http://localhost?{}", response})).unwrap().query_pairs() {
            match key {
                std::borrow::Cow::Borrowed("oauth_token") => { token = value.to_string(); },
                std::borrow::Cow::Borrowed("oauth_token_secret") => { token_secret = value.to_string(); },
                _ => {}
            }
        }

        assert_ne!(token, "".to_string());
        assert_ne!(token_secret, "".to_string());
        Ok((token, token_secret))
    }

    async fn request_pin(oauth_token: &str) -> Result<String, String> {
        let url_with_param = format!{"{}?oauth_token={}", API_AUTHORIZE_URL, urlencoding::encode(oauth_token)};

        println!{"\x1b[1;97mPlease open the URL and confirm access:\x1b[97m {}", url_with_param}

        print!{"\x1b[1;97mPlease input the PIN number:\x1b[97m "};
        io::stdout().flush().unwrap();
        let mut raw_pin = String::new();
        io::stdin().read_line(&mut raw_pin).unwrap();
        let pin = raw_pin.replace("\n", "");

        let validater = pin.clone();
        let is_valid_pin = validater.len() == 7 && !validater.chars().map(|c| c.is_numeric()).collect::<Vec<bool>>().contains(&false);

        return if is_valid_pin { Ok(pin) } else { Err("Invalid PIN".to_string()) } 
    }

    async fn request_access_token(unverified_token: &str, verifier: &str) -> Result<(String, String), String> {
        let request_url = format!{"{}?oauth_token={}&oauth_verifier={}", API_ACCESS_TOKEN_URL, unverified_token, verifier};

        let request_res = reqwest::Client::new()
            .post(request_url)
            .send().await.unwrap();

        let params = request_res.text().await.unwrap();
        let parse_url = Url::parse(&(format!{"http://localhost?{}", params})).unwrap();
        let mut access_token = String::new();
        let mut access_token_secret = String::new();
        for (key, value) in parse_url.query_pairs() {
            match key {
                std::borrow::Cow::Borrowed("oauth_token") => { access_token = value.to_string(); },
                std::borrow::Cow::Borrowed("oauth_token_secret") => { access_token_secret = value.to_string(); },
                _ => {}
            }
        }

        assert_ne!(access_token, String::new());
        assert_ne!(access_token_secret, String::new());

        Ok((access_token, access_token_secret))
    }

    async fn perform_auth(apikey: &str, apisecret: &str) -> Result<(String, String), Box<dyn Error + Send + Sync + 'static>> {
        let token_result = Self::request_token(apikey, apisecret);
        let (token, _token_secret) = token_result.await?;
        let pin = Self::request_pin(&token).await?;
        let (access_token, access_token_secret) = Self::request_access_token(&token, &pin).await?;

        println!{"\x1b[1;32mAPI access granted.\x1b[0;97m"}
        Ok((access_token, access_token_secret))
    }

    pub async fn post(
        &self,
        request: reqwest::RequestBuilder,
        url: &Url,
        params: &BTreeMap<String, String>,
        oauth_params: &BTreeMap<String, String>
    ) -> Result<reqwest::Response, reqwest::Error> {
        let mut oauth_params_for_header = BTreeMap::new();
        for (k, v) in oauth_params {
            oauth_params_for_header.insert(k.clone(), v.clone());
        }
    
        if self.oauth_token.is_some() {
            oauth_params_for_header.insert(
                "oauth_token".to_string(),
                self.oauth_token
                    .as_ref()
                    .to_owned()
                    .unwrap()
                    .to_string()
            );
        }
        
        let header_factory = OAuthHeader::new(
            &self.api_key,
            &self.api_secret,
            "POST",
            url,
            params,
            &oauth_params_for_header
        );

        request.header("Authorization", header_factory.header).send().await
    }

    /*
    pub async fn get(
        &self,
        request: reqwest::RequestBuilder,
        url: &Url,
        params: &BTreeMap<String, String>,
        oauth_params: &BTreeMap<String, String>
    ) -> Result<reqwest::Response, reqwest::Error> {
        let header_factory = OAuthHeader::new(
            &self.api_key,
            &self.api_secret,
            "GET",
            url,
            params,
            oauth_params
        );

        request.header("Authorization", header_factory.header).send().await
    }
    */
}
