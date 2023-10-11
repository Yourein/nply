mod main_logics;

use dotenv;
use main_logics::post_current_song;
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

async fn get_spotify_api(cred: &AuthKey) -> SpotifyAPI {
    match SpotifyAPI::new(&cred.spotify_key, &cred.spotify_secret).await {
        Ok(inst) => {
            inst
        }
        Err(reason) => {
            panic!("{}", reason)
        }
    }
}

#[tokio::main]
async fn main() {
    let credentials = get_credentials();

    //creating API instances
    let tapi = TwitterAPI::new(&credentials.twitter_key, &credentials.twitter_secret).await;
    let mut sapi = get_spotify_api(&credentials).await;

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
                        Err(e) => {
                            eprintln!{"Error!\nExpected reason: {}", e};
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
