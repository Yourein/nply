#[cfg(test)]
mod tests {
    use crate::api::MisskeyApi;
    use crate::types::{*, Responses::*};
    use crate::consts::ENDPOINT;

    use bytes::Bytes;
    use std::fs;

    #[test]
    fn endpoint_url_formatting01() {
        let mapi = MisskeyApi::new("dummycred".to_string(), "post.yourein.net".to_string());
        let res = mapi.get_endpoint_url(ENDPOINT::notes::create);
        
        assert_eq!(res, "https://post.yourein.net/api/notes/create".to_string());
    }

    #[test]
    fn endpoint_url_formatting02() {
        let mapi = MisskeyApi::new("dummycred".to_string(), "test.hoge.jp".to_string());
        let res = mapi.get_endpoint_url(ENDPOINT::drive::files::create);
        
        assert_eq!(res, "https://test.hoge.jp/api/drive/files/create".to_string());
    }

    #[test]
    fn classify_img_jpeg() {
        let mapi = MisskeyApi::new("dummycred".to_string(), "test.hoge.jp".to_string());
        
        let file = fs::read("./src/test/assets/yourein_test.jpeg").unwrap();
        let pic: Bytes = Bytes::from(file);
        let res = mapi.get_image_mime_type(&pic);
        assert_eq!(res, "image/jpeg");
    }

    #[test]
    fn classify_img_png() {
        let mapi = MisskeyApi::new("dummycred".to_string(), "test.hoge.jp".to_string());
        
        let file = fs::read("./src/test/assets/yourein_test.png").unwrap();
        let pic: Bytes = Bytes::from(file);
        let res = mapi.get_image_mime_type(&pic);
        assert_eq!(res, "image/png");
    }
}
