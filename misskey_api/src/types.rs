pub struct CreatingNote {
    /// A type of request body of notes/create
    /// Please notice that this is very limited difinition.
    /// You can extend this type to change other note options.
    ///
    /// To find complete definition, please read https://post.yourein.net/api-doc#tag/notes/operation/notes/create

    pub text: String,         // Body
    pub mediaIds: Vec<String> // Pictures? (I don't know the difference between 'fileIds: Vec<String>')
}

/// A type represents a misskey user
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
pub mod Responses {
    use super::User;

    /// Response of notes/create 200 (OK)
    /// Please notice that this is an **INCOMPLETE** definition.
    /// Some optional parameters could be dropped at parse.
    ///
    /// To find complete definition, please read https://post.yourein.net/api-doc#tag/notes/operation/notes/create
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

    pub struct CommonError {
        //! A common type for an error response

        pub code: String,
        pub message: String,
        pub id: String
    }
}
