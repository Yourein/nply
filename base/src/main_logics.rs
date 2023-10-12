use interface::PostAPI;
use spotify_api::api_model::api::Api as SpotifyAPI;
use spotify_api::api_model::responses::CurrentSong;

pub async fn post_current_song_on_local(
    data: CurrentSong,
    papi: &dyn PostAPI
) -> Result<(), String> {
    let artists = &data.track_artists.join(", ");
    let text = if &artists.chars().count() > &0 {
        format!{"#nowplaying {} - {}", &data.song_title, artists}
    }
    else {
        format!{"#nowplaying {}", &data.song_title}
    };

    match papi.compose_without_picture(&text).await {
        Ok(_) => Ok(()),
        Err(_) => Err("Tweet failed.".to_string())
    }
}

pub async fn post_current_song_on_spotify(
    data: CurrentSong,
    papi: &dyn PostAPI
) -> Result<(), String> {
    let img = reqwest::get(&data.album_art_url.to_owned().unwrap()).await
        .unwrap()
        .bytes().await
        .unwrap();

    match papi.upload_media(img).await {
        Some(id) => {
            let media_ids = vec![id];
            let song_url = format!{"https://open.spotify.com/track/{}", &data.song_uri.to_owned().unwrap()};
            let artists = &data.track_artists.join(", ");
            let text = format!{
                "#nowplaying {} - {}\n{}",
                &data.song_title,
                artists,
                song_url
            };

            let _ = papi.compose_with_picture(&text, &media_ids).await;
            Ok(())
        }
        None => {
            Err("Tweet failed. Could not upload a image to twitter".to_string())
        }
    }
}

pub async fn post_current_song(papi: &dyn PostAPI, sapi: &mut SpotifyAPI<'_>) {
    match sapi.fetch_current_song().await {
        Ok(raw) => {
            let resp = sapi.parse_current_song_result(raw);

            let post_result = if resp.album_art_url.is_none() {
                post_current_song_on_local(resp, papi).await
            }
            else {
                post_current_song_on_spotify(resp, papi).await
            };

            match post_result {
                Ok(_) => println!{"Posted Successfully."},
                Err(reason) => eprintln!{"Error!\nExpected reason: {}", reason.to_string()}
            }
        }
        Err(reason) => {
            eprintln!{"Error!\nExpected reason: {}", reason.to_string()};
        }
    }
}
