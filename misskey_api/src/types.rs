use serde::{Serialize, Deserialize};

/// A type of request body of notes/create
/// Please notice that this is very limited difinition.
/// You can extend this type to change other note options.
///
/// To find complete definition, please read https://post.yourein.net/api-doc#tag/notes/operation/notes/create
#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
pub struct CreatingNote {
    pub text: String,         // Body
    pub mediaIds: Vec<String> // Pictures? (I don't know the difference between 'fileIds: Vec<String>')
}

/// A type represents a misskey user
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct User {

    pub id: String,
    pub name: Option<String>,
    pub username: String,
    pub host: Option<String>, //The local host is represented with None.
    pub avatarUrl: Option<String>,
    pub avatarBlurhash: Option<String>,
    pub isAdmin: Option<bool>,
    pub isModerator: Option<bool>,
    pub isBot: Option<bool>,
    pub isCat: Option<bool>,
    pub onlineStatus: Option<String>
}

/// Responses of Misskey backend (API)
#[allow(non_snake_case)]
pub mod Responses {
    use super::User;
    use serde::Deserialize;
    
    /// Response of notes/create 200 (OK)
    /// Please notice that this is an **INCOMPLETE** definition.
    /// Some optional parameters could be dropped at parse.
    ///
    /// To find complete definition, please read https://post.yourein.net/api-doc#tag/notes/operation/notes/create
    #[derive(Deserialize, Debug)]
    #[allow(non_snake_case)]
    pub struct CreatedNote {
        pub id: String,
        pub createdAt: String,
        pub text: Option<String>,
        pub cw: Option<String>,
        pub userId: String,
        pub user: User,
        pub visibility: String,
        pub uri: Option<String>,
        pub url: Option<String>
    }

    /// A type that represents a file in drive
    #[derive(Deserialize, Debug)]
    #[allow(non_snake_case)]
    pub struct DriveFile {
        pub id: String,
        pub name: String,
        #[serde(rename = "type")]
        pub format: String,
        pub md5: String,
        pub size: i32,
        pub isSensitive: bool,
        pub blurhash: Option<String>,
        pub properties: ImgProperties,
        pub url: Option<String>,
        pub thumbnailUrl: Option<String>,
        pub comment: Option<String>,
        pub folderId: Option<String>,
        pub folder: Option<DriveFolder>,
        pub userId: Option<String>,
        pub user: Option<User>
    }

    #[derive(Deserialize, Debug)]
    #[allow(non_snake_case)]
    pub struct ImgProperties {
        pub width: i32,
        pub height: i32,
        pub orientation: i32,
        pub avgColor: String
    }

    /// A type that represents a folder in drive
    #[derive(Deserialize, Debug)]
    #[allow(non_snake_case)]
    pub struct DriveFolder {
        pub id: String,
        pub createdAt: String,
        pub name: String,
        pub foldersCount: Option<i32>,
        pub filesCount: Option<i32>,
        pub parentId: Option<String>,
        pub parent: Option<Box<DriveFile>>
    }

    /// A common type for an error response
    #[derive(Deserialize, Debug)]
    #[allow(non_snake_case)]
    pub struct CommonError {
        pub code: String,
        pub message: String,
        pub id: String
    }
}
