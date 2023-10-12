use std::collections::BTreeMap;
use chrono::Local;
use urlencoding::encode as p_encode;
use std::str;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use url::Url;
use base64::{Engine as _, engine::general_purpose};

pub struct OAuthHeader {
    pub signature: String,
    pub header: String
}

impl OAuthHeader {
    pub fn new (
        api_key: &str,
        api_secret: &str,
        token_secret: &str,
        request_method: &str,
        request_url: &Url,
        extra_params: &BTreeMap<String, String>,
        extra_oauth_params: &BTreeMap<String, String>
    ) -> OAuthHeader {
        //パラメータの準備
        let nonce = Self::create_oauth_nonce();
        let current_timestamp = Local::now().timestamp().to_string();
        let crypt_key = format!{"{}&{}", p_encode(api_secret), p_encode(token_secret)};


        let mut params: BTreeMap<String, String> = BTreeMap::new();
        for (key, val) in extra_params {
            params.insert(key.to_string(), val.to_string());
        }
        for (key, val) in extra_oauth_params {
            params.insert(key.to_string(), val.to_string());
        }
        params.insert("oauth_consumer_key".to_string(), api_key.to_string());
        params.insert("oauth_nonce".to_string(), nonce.clone());
        params.insert("oauth_signature_method".to_string(), "HMAC-SHA1".to_string());
        params.insert("oauth_timestamp".to_string(), current_timestamp.clone());
        params.insert("oauth_version".to_string(), "1.0".to_string());

        //signatureの作成
        let signature_string = Self::create_signature(
            request_method.to_string(),
            request_url,
            &params,
            crypt_key
        );

        let mut header_params: BTreeMap<String, String> = BTreeMap::new();
        header_params.insert("oauth_consumer_key".to_string(), api_key.to_string());
        header_params.insert("oauth_nonce".to_string(), nonce.clone());
        header_params.insert("oauth_signature".to_string(), signature_string.clone());
        header_params.insert("oauth_signature_method".to_string(), "HMAC-SHA1".to_string());
        header_params.insert("oauth_timestamp".to_string(), current_timestamp.clone());
        header_params.insert("oauth_version".to_string(), "1.0".to_string());
        for (key, val) in extra_oauth_params {
            header_params.insert(key.to_string(), val.to_string());
        }
        
        //authorization headerの作成
        let header_string = Self::create_authorization_header(&header_params);
        
        OAuthHeader {
            signature: signature_string,
            header: header_string
        }
    }

    fn create_authorization_header(params: &BTreeMap<String, String>) -> String {
        let param_string = params
            .iter()
            .map(|(k, v)| format!{r#"{}="{}""#, k, p_encode(v)})
            .collect::<Vec<String>>()
            .join(", ");

        format!{"OAuth {}", param_string}
    }

    fn create_signature(request_method: String, url: &Url, params: &BTreeMap<String, String>, crypt_key: String) -> String {
        let param_string = params.iter().map(|(k, v)| format!{"{}={}", k, p_encode(v)}).collect::<Vec<String>>().join("&");
        let base = format!{"{}&{}&{}", request_method, p_encode(&(Self::get_base_url(url))), p_encode(&param_string)};
        let hash = hmacsha1::hmac_sha1(crypt_key.as_bytes(), base.as_bytes());
        
        general_purpose::STANDARD_NO_PAD.encode(&hash)
    }

    fn get_base_url(url: &Url) -> String {
        format!{
            "{}://{}{}",
            url.scheme(),
            url.host_str().unwrap_or(""),
            url.path()
        }
    }

    fn create_oauth_nonce() -> String {
        let rand_str: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        let b64_encoded_str = general_purpose::STANDARD_NO_PAD.encode(rand_str);
        let res = b64_encoded_str.replace(&['+', '/', '='], "");

        res
    }
}
