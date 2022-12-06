mod api_model;
use tokio;
use api_model::api::Api;
use api_model::credentials::OAuth1::ApiCredential;

#[tokio::main]
async fn main() {
    let mut inst = ApiCredential::new("", "");

    match inst.request_token().await {
        Ok(_) => println!{"OK"},
        Err(_) => println!{"Error"}
    }
}
