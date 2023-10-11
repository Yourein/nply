#[cfg(test)]
mod tests {
    use crate::api::MisskeyApi;
    use crate::types::{*, Responses::*};
    use crate::consts::ENDPOINT;

    #[test]
    fn endpoint_url_formatting01() {
        let mapi = MisskeyApi::new("dummycred".to_string(), "test.hoge.jp".to_string());
        let res = mapi.get_endpoint_url(ENDPOINT::notes::create);
        
        assert_eq!(res, "https://test.hoge.jp/notes/create".to_string());
    }

    #[test]
    fn endpoint_url_formatting02() {
        let mapi = MisskeyApi::new("dummycred".to_string(), "test.hoge.jp".to_string());
        let res = mapi.get_endpoint_url(ENDPOINT::drive::files::create);
        
        assert_eq!(res, "https://test.hoge.jp/drive/files/create".to_string());
    }
}
