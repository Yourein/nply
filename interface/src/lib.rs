use async_trait::async_trait;
use bytes::Bytes;

/// An API functions for compose posting is represented here
#[async_trait]
pub trait PostAPI {
    async fn upload_media(
        &self, picture: Bytes
    ) -> Option<String>;
    
    async fn compose_without_picture(
        &self, text: &str
    ) -> String;
    
    async fn compose_with_picture(
        &self, text: &str, media: &Vec<String>
    ) -> String;
}
