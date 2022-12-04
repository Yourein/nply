mod api_model;
use api_model::api::Api;
use api_model::credentials::Code;

#[tokio::main]
async fn main() {
    let cid: &str = "";
    let sid: &str = "";
    
    let mut api: Api = Api::new(cid, sid);

    match api.get_current_song().await {
        Ok(s) => println!{"{}", s},
        Err(_) => ()
    }
}
