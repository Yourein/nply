use dotenv;
use reqwest;
use twitter_api::api_model::api::Api as TwitterAPI;
use spotify_api::api_model::api::Api as SpotifyAPI;
use std::io::stdin;
use std::io::{stdout, Write};

struct AuthKey {
    spotify_key: String,
    spotify_secret: String,
    twitter_key: String,
    twitter_secret: String 
}

fn get_credentials() -> AuthKey {
    dotenv::dotenv().ok();
    let spotify_key = dotenv::var("SPOTIFY_API_KEY").unwrap();
    let spotify_secret = dotenv::var("SPOTIFY_API_SECRET").unwrap();
    let twitter_key = dotenv::var("TWITTER_API_KEY").unwrap();
    let twitter_secret = dotenv::var("TWITTER_API_SECRET").unwrap();

    AuthKey {
        spotify_key: spotify_key,
        spotify_secret: spotify_secret,
        twitter_key: twitter_key,
        twitter_secret: twitter_secret
    }
}

async fn post_current_song(tapi: &TwitterAPI, sapi: &mut SpotifyAPI<'_>) -> Result<(), String> {
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
        
        return Ok(());
    }
    else {
        return Err("Tweet failed. Please check if you playing a song on Spotify now.".to_string());
    }
}

#[tokio::main]
async fn main() {
    let credentials = get_credentials();
    
    //creating API instances
    let mut sapi = SpotifyAPI::new(&credentials.spotify_key, &credentials.spotify_secret);
    let tapi = TwitterAPI::new(&credentials.twitter_key, &credentials.twitter_secret).await;

    println!{"\x1b[1;32mReady\x1b[0;97m"};
    
    loop {
        print!{"Waiting... (0) exit (1) Post current song : "};
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        
        if let Ok(i) = input.trim().parse::<usize>() {
            match i {
                0 => {
                    std::process::exit(0);
                }
                1 => {
                    match post_current_song(&tapi, &mut sapi).await {
                        Ok(()) => {
                            println!{"Posted Successfully"};
                        }
                        _ => {
                            eprintln!{"Unknown Error"};
                        }
                    }
                },
                _ => {
                    eprintln!{"Unknown choice selected. Please try again"};
                }
            }
        }
        else {
            eprintln!{"Invalid input. Please try again"};
        }
        println!{};
    }
}
