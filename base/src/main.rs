use dotenv;
use reqwest;

#[tokio::main]
async fn main() {
    //fetch secrets
    dotenv::dotenv().ok();
    let spotify_key = dotenv::var("SPOTIFY_API_KEY").unwrap();
    let spotify_secret = dotenv::var("SPOTIFY_API_SECRET").unwrap();
    let twitter_key = dotenv::var("TWITTER_API_KEY").unwrap();
    let twitter_secret = dotenv::var("TWITTER_API_SECRET").unwrap();

    //creating API instances
    let mut sapi = spotify_api::api_model::api::Api::new(&spotify_key, &spotify_secret);
    let tapi = twitter_api::api_model::api::Api::new(&twitter_key, &twitter_secret).await;
    
    //fetch current playing
    let resp = sapi.get_current_song().await.unwrap();
    
    let img = reqwest::get(&resp.item.album.images[0].url).await.unwrap().bytes().await.unwrap();
    let media_id = tapi.upload_picture(img).await;

    if media_id.is_some() {
        let mut media_ids: Vec<String> = Vec::new();
        media_ids.push(media_id.unwrap());

        let title = resp.item.name;
        let song_uri = resp.item.uri.strip_prefix("spotify:track:").unwrap();
        let song_url = format!{"https://open.spotify.com/track/{}", song_uri};
        
        //Put artists into a single String
        let mut artists_vec: Vec<String> = Vec::new();
        for x in &resp.item.artists {
            artists_vec.push(x.name.clone());
        }
        let artists = artists_vec.join(", ");
        
        let text = format!{"#nowplaying {} - {}\n{}", title, artists, song_url};
        let _ = tapi.compose_new_tweet_with_media(&text, &media_ids).await;
    }
    else {
        println!{"Tweet failed. Please check if you playing a song on Spotify now."}
    }
}
