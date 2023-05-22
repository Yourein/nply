use webbrowser;

pub async fn throw_url(carg: &str) -> Result<(), String> {
    match webbrowser::open(carg) {
        Ok(_) => Ok(()),
        Err(_) => {
            let reason = format!{"Could not open the URL: {} in your browser", carg};
            Err(reason)
        }
    }
}
