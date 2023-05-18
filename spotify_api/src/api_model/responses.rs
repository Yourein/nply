#[allow(non_snake_case)]
pub mod CurrentlyPlaying {
    use std::collections::HashMap;
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Device {
        pub id: String,
        pub is_active: bool,
        pub is_private_session: bool,
        pub is_restricted: bool,
        pub name: String,

        #[serde(rename = "type")]
        pub type_id: String,
        pub volume_percent: i32
    }

    #[derive(Deserialize, Debug)]
    pub struct Context {
        #[serde(rename = "type")]
        pub type_id: String,
        pub href: String,
        pub external_urls: HashMap<String, String>,
        pub uri: String
    }

    #[derive(Deserialize, Debug)]
    pub struct Artists {
        pub external_urls: HashMap<String, String>,
        pub href: String,
        pub id: String,
        pub name: String,

        #[serde(rename = "type")]
        pub type_id: String,
        pub uri: String
    }

    #[derive(Deserialize, Debug)]
    pub struct Image {
        pub width: i64,
        pub height: i64,
        pub url: String
    }

    #[derive(Deserialize, Debug)]
    pub struct Album {
        pub album_type: String,
        pub artists: Vec<Artists>,
        pub available_markets: Vec<String>,
        pub external_urls: HashMap<String, String>,
        pub href: String,
        pub id: String,
        pub images: Vec<Image>,
        pub name: String,
        pub release_date: String,
        pub release_date_precision: String,
        pub total_tracks: i32,
        #[serde(rename = "type")]
        pub type_id: String,
        pub uri: String
    }

    #[derive(Deserialize, Debug)]
    pub struct Item {
        pub album: Album,
        pub artists: Vec<Artists>,
        pub available_markets: Vec<String>,
        pub disc_number: i32,
        pub duration_ms: i32,
        pub explicit: bool,
        pub external_ids: HashMap<String, String>,
        pub external_urls: HashMap<String, String>,
        pub href: String,
        pub id: String,
        pub is_local: bool,
        pub name: String,
        pub popularity: i32,
        pub preview_url: Option<String>,
        pub track_number: i32,
        #[serde(rename = "type")]
        pub type_id: String,
        pub uri: String
    }

    #[derive(Deserialize, Debug)]
    pub struct Actions {
        pub disallows: HashMap<String, bool>
    }
    
    #[derive(Deserialize, Debug)]
    pub struct Responses {
        pub device: Option<Device>,
        pub repeat_state: Option<String>,
        pub shuffle_state: Option<String>,
        pub context: Option<Context>,
        pub timestamp: i64,
        pub progress_ms: i64,
        pub item: Item,
        pub currently_playing_type: String,
        pub actions: Actions,
        pub is_playing: bool
    }
}

pub struct CurrentSong {
    pub song_title: String,
    pub song_uri: String,
    pub track_artists: Vec<String>,
    pub album_title: String,
    pub album_artists: Vec<String>,
    pub album_art_url: String
}
