use dotenv;

fn main() {
    dotenv::dotenv().ok();

    let spotify_key = dotenv::var("SPOTIFY_API_KEY").unwrap();
    let spotify_secret = dotenv::var("SPOTIFY_API_SECRET").unwrap();
    let twitter_key = dotenv::var("TWITTER_API_KEY").unwrap();
    let twitter_secret = dotenv::var("TWITTER_API_SECRET").unwrap();

    println!{
        "Spotify key: {}\nSpotify secret: {}\nTwitter key: {}\nTwitter secret: {}",
        spotify_key,
        spotify_secret,
        twitter_key,
        twitter_secret
    };
}
