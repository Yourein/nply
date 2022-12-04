mod api_model;
use api_model::api::Api;

#[tokio::main]
async fn main() {
    let cid: &str = "";
    let sid: &str = "";

    let mut inst = Api::new(cid, sid);

    match inst.get_current_song().await {
        Ok(s) => println!{"{}", s},
        Err(_) => println!{"Error!"}
    }
}
