const WEB_URL: &str = "https://linear.app/";
const APP_URL: &str = "linear://";

pub struct LinearClient {
    base_url: &'static str,
}

impl LinearClient {
    pub fn new(use_app: bool) -> Self {
        let base_url = if use_app { APP_URL } else { WEB_URL };
        Self { base_url }
    }

    pub fn view(&self, organization: &str, issue: &str) {
        let base_url = self.base_url;
        let url = format!("{base_url}{organization}/issue/{issue}");
        open::that(url).unwrap();
    }
}
